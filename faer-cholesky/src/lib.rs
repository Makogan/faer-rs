use assert2::assert as fancy_assert;
use core::cmp::Ordering;
use faer_core::permutation::PermutationIndicesMut;
use faer_core::MatRef;
use num_traits::Signed;

pub mod llt;
pub mod ldlt;

#[track_caller]
#[inline]
unsafe fn unreachable_unchecked() -> ! {
    debug_assert!(false);
    core::hint::unreachable_unchecked()
}

/// Computes a permutation that reduces the chance of numerical errors during the cholesky
/// factorization, then stores the result in `perm_indices` and `perm_inv_indices`.
#[track_caller]
pub fn compute_cholesky_permutation<'a, T>(
    perm_indices: &'a mut [usize],
    perm_inv_indices: &'a mut [usize],
    matrix: MatRef<'_, T>,
) -> PermutationIndicesMut<'a>
where
    T: Signed + PartialOrd,
{
    let n = matrix.nrows();
    fancy_assert!(
        matrix.nrows() == matrix.ncols(),
        "input matrix must be square",
    );
    fancy_assert!(
        perm_indices.len() == n,
        "length of permutation must be equal to the matrix dimension",
    );
    fancy_assert!(
        perm_inv_indices.len() == n,
        "length of inverse permutation must be equal to the matrix dimension",
    );

    let diag = matrix.diagonal();
    for (i, p) in perm_indices.iter_mut().enumerate() {
        *p = i;
    }

    perm_indices.sort_unstable_by(move |&i, &j| {
        let lhs = unsafe { diag.get_unchecked(i) }.abs();
        let rhs = unsafe { diag.get_unchecked(j) }.abs();
        let cmp = rhs.partial_cmp(&lhs);
        if let Some(cmp) = cmp {
            cmp
        } else {
            Ordering::Equal
        }
    });

    for (i, p) in perm_indices.iter().copied().enumerate() {
        *unsafe { perm_inv_indices.get_unchecked_mut(p) } = i;
    }

    unsafe { PermutationIndicesMut::new_unchecked(perm_indices, perm_inv_indices) }
}
