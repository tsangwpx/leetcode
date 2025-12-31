// Problem 214
impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        /*
         * Build KMP on aux string, s + "#" + s.rev()
         *
         * So that we may figure out the longest palindrome starting from the beginning in linear time
         * because KMP table tells the maximum proper prefix length which is also suffix
         */
        let s = s.into_bytes();

        // aux = s + "#" + reverse(s)
        let mut aux = Vec::with_capacity(s.len() * 2 + 1);
        aux.extend(s.iter());
        aux.push(b'#');
        aux.extend(s.iter().rev());

        // KMP
        let mut table = vec![0; aux.len()];
        let mut len = 0;
        let mut idx = 1;

        while idx < aux.len() {
            if aux[idx] == aux[len] {
                // character matches! bump length of proper prefix which is also suffix
                len += 1;
                table[idx] = len;

                idx += 1;
            } else {
                // no match found!
                // check if we may fall back to previous location

                if len == 0 {
                    // no: advance the pointer
                    idx += 1;
                } else {
                    // yes: try another round
                    len = table[len - 1];
                }
            }
        }

        // the last item tells the length of prefix which is also suffix of the aux string
        // or the max length of palindrome starting from the beginning in the original string
        let longest = table.last().copied().unwrap();
        let missing = s.len() - longest;
        let total = missing + s.len();

        let mut buf = Vec::with_capacity(total);

        buf.extend(s.iter().rev().take(missing));
        buf.extend(s.iter());

        String::from_utf8(buf).unwrap()
    }
}
