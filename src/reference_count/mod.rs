use std::collections::HashMap;

use crate::{
    base::{symbol::Symbol, Primitive},
    env::Env,
    lower_ir::{
        layout::{LowerLayout, LowerLayoutId},
        LowerIR,
    },
};

pub mod stmt;

// TODO: explain what this stage should do and a bit of how
pub fn reference_count(_lower_ir: &LowerIR, _env: &mut Env) -> RefCountIR {
    todo!()
}

pub struct RefCountIR {}

// IDEA: use the mono2 strategy of having a BorrowSignatureCache and calculate said BorrowSignature when needed.
// This avoids the worry that we won't have calculated a borrow signature for something while still avoiding unnecessary work.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ownership {
    Owned,
    Borrowed,
}

fn layout_to_ownership<'a>(layout_id: LowerLayoutId, lower_ir: &LowerIR) -> Ownership {
    match lower_ir[layout_id] {
        LowerLayout::List(_) | LowerLayout::Primitive(Primitive::Str) => Ownership::Borrowed,
        _ => Ownership::Owned,
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct BorrowSignature(u64);

impl std::fmt::Debug for BorrowSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = &mut f.debug_struct("BorrowSignature");

        for (i, ownership) in self.iter().enumerate() {
            f = f.field(&format!("_{i}"), &ownership);
        }

        f.finish()
    }
}

impl BorrowSignature {
    fn new(len: usize) -> Self {
        assert!(len < 64 - 8);

        Self(len as _)
    }

    // fn from_layouts<'a>(
    //     // interner: &impl LayoutInterner<'a>,
    //     // layouts: impl ExactSizeIterator<Item = &'a InLayout<'a>>,
    //     layout_cache: (),
    // ) -> Self {
    //     let mut signature = BorrowSignature::new(layouts.len());

    //     for (i, layout) in layouts.enumerate() {
    //         signature.set(i, layout_to_ownership(*layout, interner));
    //     }

    //     signature
    // }

    fn len(&self) -> usize {
        (self.0 & 0xFF) as usize
    }

    fn get(&self, index: usize) -> Option<&Ownership> {
        if index >= self.len() {
            return None;
        }

        match self.0 & (1 << (index + 8)) {
            0 => Some(&Ownership::Borrowed),
            _ => Some(&Ownership::Owned),
        }
    }

    fn set(&mut self, index: usize, ownership: Ownership) -> bool {
        assert!(index < self.len());

        let modified = self.get(index) != Some(&ownership);

        let mask = 1 << (index + 8);

        match ownership {
            Ownership::Owned => self.0 |= mask,
            Ownership::Borrowed => self.0 &= !mask,
        }

        modified
    }

    pub fn iter(&self) -> impl Iterator<Item = Ownership> + '_ {
        let mut i = 0;

        std::iter::from_fn(move || {
            let value = self.get(i)?;
            i += 1;
            Some(*value)
        })
    }
}

impl std::ops::Index<usize> for BorrowSignature {
    type Output = Ownership;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

pub struct BorrowSignatureCache {
    signatures_by_proc_symbol: HashMap<Symbol, BorrowSignature>,
}

// pub(crate) fn infer_borrow_signatures(lower_ir: &LowerIR) -> BorrowSignatures {}

// pub(crate) struct BorrowSignatures<'a> {
//     pub(crate) procs: MutMap<(Symbol, ProcLayout<'a>), BorrowSignature>,
// }

//     let mut borrow_signatures: BorrowSignatures = BorrowSignatures {
//         procs: procs
//             .iter()
//             .map(|(_key, proc)| {
//                 let key = (proc.name.name(), proc.proc_layout(arena));
//                 let signature = BorrowSignature::from_layouts(interner, key.1.arguments.iter());
//                 (key, signature)
//             })
//             .collect(),
//     };

//     // for every proc (by index) a collection of its join points
//     let mut join_points: Vec<_> = std::iter::repeat_with(MutMap::default)
//         .take(procs.len())
//         .collect_in(arena);

//     // next we first partition the functions into strongly connected components, then do a
//     // topological sort on these components, finally run the fix-point borrow analysis on each
//     // component (in top-sorted order, from primitives (std-lib) to main)

//     let matrix = construct_reference_matrix(arena, procs);
//     let sccs = matrix.strongly_connected_components_all();

//     let mut join_point_stack = Vec::new_in(arena);
//     let mut proc_join_points = MutMap::default();

//     for (group, _) in sccs.groups() {
//         // This is a fixed-point analysis
//         //
//         // all functions initially own all their parameters
//         // through a series of checks and heuristics, some arguments are set to borrowed
//         // when that doesn't lead to conflicts the change is kept, otherwise it may be reverted
//         //
//         // when the signatures no longer change, the analysis stops and returns the signatures

//         loop {
//             let mut modified = false;

//             for index in group.iter_ones() {
//                 let (_, proc) = procs.iter().nth(index).unwrap();
//                 let key = (proc.name.name(), proc.proc_layout(arena));

//                 if proc.args.is_empty() {
//                     continue;
//                 }

//                 std::mem::swap(&mut proc_join_points, &mut join_points[index]);

//                 let mut state = State {
//                     args: proc.args,
//                     borrow_signature: *borrow_signatures.procs.get(&key).unwrap(),
//                     join_point_stack,
//                     join_points: proc_join_points,
//                     modified: false,
//                 };

//                 state.inspect_stmt(interner, &mut borrow_signatures, &proc.body);

//                 // did any proc signature get modified?
//                 //
//                 // NOTE: this does not directly include updates to join point signatures. The
//                 // assumption is that a relevant change in join point signature is immediately
//                 // (i.e. no fixpoint is required) reflected in the proc signature.
//                 //
//                 // TODO: this is a load-bearing assert! There must be UB somewhere, removing this
//                 // assert causes the code to run into an infinite loop that terminates when the
//                 // memory on the system is exhausted.
//                 assert_eq!(
//                     state.modified,
//                     borrow_signatures
//                         .procs
//                         .insert(key, state.borrow_signature)
//                         .unwrap()
//                         != state.borrow_signature
//                 );
//                 modified |= state.modified;

//                 proc_join_points = state.join_points;

//                 std::mem::swap(&mut proc_join_points, &mut join_points[index]);

//                 join_point_stack = state.join_point_stack;
//                 join_point_stack.clear();
//             }

//             if !modified {
//                 break;
//             }
//         }
//     }

//     borrow_signatures
// }
