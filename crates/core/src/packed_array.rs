#[derive(Debug, Clone, PartialEq)]
pub struct PackedArray {
    length: usize,
    bits_per_value: usize,
    values_per_u64: usize,
    max_value: u64,
    index_scale: u32,
    index_offset: u32,
    index_shift: usize,
    pub bits: Vec<u64>,
}

impl PackedArray {
    pub fn new(length: usize, bits_per_value: usize) -> PackedArray {
        let values_per_u64 = 64 / bits_per_value;
        let max_value = (1u64 << bits_per_value) - 1;
        let i = 3 * (values_per_u64 - 1);
        let index_scale = INDEX_PARAMETERS[i] as u32;
        let index_offset = INDEX_PARAMETERS[i + 1] as u32;
        let index_shift = INDEX_PARAMETERS[i + 2] as usize;
        let need_u64s = (length * bits_per_value + 63) / 64;

        Self {
            length,
            bits_per_value,
            values_per_u64,
            max_value,
            index_scale,
            index_offset,
            index_shift,
            bits: vec![0u64; need_u64s],
        }
    }

    pub fn bits_per_value(&self) -> usize {
        self.bits_per_value
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn get(&self, index: usize) -> Option<u64> {
        if index >= self.len() {
            return None;
        }
        let i = self.get_storage_index(index);
        let l = self.bits[i];
        let j = (index - i * self.values_per_u64) * self.bits_per_value;
        Some(l >> j & self.max_value)
    }

    pub fn set(&mut self, index: usize, value: u64) {
        debug_assert!(index < self.len());
        debug_assert!(value <= self.max_value);
        let i = self.get_storage_index(index);
        let j = (index - i * self.values_per_u64) * self.bits_per_value;
        let l = &mut self.bits[i];
        *l &= !(self.max_value << j);
        *l |= value << j;
    }

    pub fn swap(&mut self, index: usize, value: u64) -> u64 {
        debug_assert!(index < self.len());
        debug_assert!(value <= self.max_value);
        let i = self.get_storage_index(index);
        let j = (index - i * self.values_per_u64) * self.bits_per_value;
        let l = &mut self.bits[i];
        let old = (*l >> j) & self.max_value;
        *l &= !(self.max_value << j);
        *l |= value << j;
        old
    }

    #[inline]
    fn get_storage_index(&self, index: usize) -> usize {
        ((index as u64) * (self.index_scale as u64) + (self.index_offset as u64) >> 32 >> self.index_shift) as usize
    }
}

const INDEX_PARAMETERS: [i32; 192] = [
    -1, -1, 0, i32::MIN, 0, 0, 1431655765, 1431655765, 0, i32::MIN, 0, 1, 858993459, 858993459, 0, 715827882, 715827882, 0, 613566756, 613566756, 0, i32::MIN, 0, 2, 477218588, 477218588, 0, 429496729, 429496729, 0, 390451572, 390451572, 0, 357913941, 357913941, 0, 330382099, 330382099, 0, 306783378, 306783378, 0, 286331153, 286331153, 0, i32::MIN, 0, 3, 252645135, 252645135, 0, 238609294, 238609294, 0, 226050910, 226050910, 0, 214748364, 214748364, 0, 204522252, 204522252, 0, 195225786, 195225786, 0, 186737708, 186737708, 0, 178956970, 178956970, 0, 171798691, 171798691, 0, 165191049, 165191049, 0, 159072862, 159072862, 0, 153391689, 153391689, 0, 148102320, 148102320, 0, 143165576, 143165576, 0, 138547332, 138547332, 0, i32::MIN, 0, 4, 130150524, 130150524, 0, 126322567, 126322567, 0, 122713351, 122713351, 0, 119304647, 119304647, 0, 116080197, 116080197, 0, 113025455, 113025455, 0, 110127366, 110127366, 0, 107374182, 107374182, 0, 104755299, 104755299, 0, 102261126, 102261126, 0, 99882960, 99882960, 0, 97612893, 97612893, 0, 95443717, 95443717, 0, 93368854, 93368854, 0, 91382282, 91382282, 0, 89478485, 89478485, 0, 87652393, 87652393, 0, 85899345, 85899345, 0, 84215045, 84215045, 0, 82595524, 82595524, 0, 81037118, 81037118, 0, 79536431, 79536431, 0, 78090314, 78090314, 0, 76695844, 76695844, 0, 75350303, 75350303, 0, 74051160, 74051160, 0, 72796055, 72796055, 0, 71582788, 71582788, 0, 70409299, 70409299, 0, 69273666, 69273666, 0, 68174084, 68174084, 0, i32::MIN, 0, 5
];

#[cfg(test)]
mod test {
    use crate::packed_array::PackedArray;

    #[test]
    fn test() {
        let values = vec![
            1, 2, 2, 3, 4, 4, 5, 6, 6, 4, 8, 0, 7, 4, 3, 13, 15, 16, 9, 14, 10, 12, 0, 2,
        ];
        let mut array = PackedArray::new(values.len(), 5);
        for (i, x) in values.iter().enumerate() {
            array.set(i, *x);
        }
        assert_eq!(0x0020863148418841, array.bits[0]);
        assert_eq!(0x01018A7260F68C87, array.bits[1]);
    }
}
