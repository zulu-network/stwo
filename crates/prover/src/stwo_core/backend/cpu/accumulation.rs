use super::CpuBackend;
use crate::stwo_core::air::accumulation::AccumulationOps;
use crate::stwo_core::fields::secure_column::SecureColumn;

impl AccumulationOps for CpuBackend {
    fn accumulate(column: &mut SecureColumn<Self>, other: &SecureColumn<Self>) {
        for i in 0..column.len() {
            let res_coeff = column.at(i) + other.at(i);
            column.set(i, res_coeff);
        }
    }
}
