//! This module provides an implementation of the [canonical Huffman] decoder.
//! 
//! [canonical Huffman]: https://en.wikipedia.org/wiki/Canonical_Huffman_code

mod error;
mod reader;
#[cfg(feature = "decode1")]
pub mod table1;
#[cfg(feature = "decode2")]
pub mod table2;
#[cfg(feature = "decode3")]
pub mod table3;
#[cfg(feature = "decode4")]
pub mod table4;
#[cfg(feature = "decode5")]
pub mod table5;

pub use error::*;
use reader::*;

/// Decodes Huffman's sequence from the provided table. The table design
/// explains how many bits should be read at the time.
/// 
/// ```rs
/// use httlib_huffman::decode;
///
/// let mut text = Vec::new();
/// let speed = 3;
/// let sequence = vec![168, 209, ...];
/// decode(&sequence, &mut text, speed).unwrap();
/// ```
pub fn decode(src: &[u8], dst: &mut Vec<u8>, speed: u8) -> Result<(), DecoderError> {
    let mut reader = DecodeReader::new(speed);

    for byte in src.iter() {
        reader.write(*byte)?;
        reader.decode(dst)?;
    }
    reader.flush()?;
    reader.decode(dst)?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    fn decode(bytes: &[u8], speed: u8) -> Result<Vec<u8>, DecoderError> {
        let mut dst = Vec::new();
        super::decode(&bytes, &mut dst, 1)?;
        Ok(dst)
    }

    fn supported_characters() -> Vec<(&'static [u8], Vec<u8>)> {
        vec![
            (&[0],   vec![255, 199]),               // 0
            (&[1],   vec![255, 255, 177]),          // 1
            (&[2],   vec![255, 255, 254, 47]),      // 2
            (&[3],   vec![255, 255, 254, 63]),      // 3
            (&[4],   vec![255, 255, 254, 79]),      // 4
            (&[5],   vec![255, 255, 254, 95]),      // 5
            (&[6],   vec![255, 255, 254, 111]),     // 6
            (&[7],   vec![255, 255, 254, 127]),     // 7
            (&[8],   vec![255, 255, 254, 143]),     // 8
            (&[9],   vec![255, 255, 234]),          // 9
            (&[10],  vec![255, 255, 255, 243]),     // 10
            (&[11],  vec![255, 255, 254, 159]),     // 11
            (&[12],  vec![255, 255, 254, 175]),     // 12
            (&[13],  vec![255, 255, 255, 247]),     // 13
            (&[14],  vec![255, 255, 254, 191]),     // 14
            (&[15],  vec![255, 255, 254, 207]),     // 15
            (&[16],  vec![255, 255, 254, 223]),     // 16
            (&[17],  vec![255, 255, 254, 239]),     // 17
            (&[18],  vec![255, 255, 254, 255]),     // 18
            (&[19],  vec![255, 255, 255, 15]),      // 19
            (&[20],  vec![255, 255, 255, 31]),      // 20
            (&[21],  vec![255, 255, 255, 47]),      // 21
            (&[22],  vec![255, 255, 255, 251]),     // 22
            (&[23],  vec![255, 255, 255, 63]),      // 23
            (&[24],  vec![255, 255, 255, 79]),      // 24
            (&[25],  vec![255, 255, 255, 95]),      // 25
            (&[26],  vec![255, 255, 255, 111]),     // 26
            (&[27],  vec![255, 255, 255, 127]),     // 27
            (&[28],  vec![255, 255, 255, 143]),     // 28
            (&[29],  vec![255, 255, 255, 159]),     // 29
            (&[30],  vec![255, 255, 255, 175]),     // 30
            (&[31],  vec![255, 255, 255, 191]),     // 31            
            (b" ",  vec![83]),                      // 32
            (b"!",  vec![254, 63]),                 // 33
            (b"\"", vec![254, 127]),                // 34
            (b"#",  vec![255, 175]),                // 35
            (b"$",  vec![255, 207]),                // 36
            (b"%",  vec![87]),                      // 33
            (b"&",  vec![248]),                     // 33
            (b"'",  vec![255, 95]),                 // 33
            (b"(",  vec![254, 191]),                // 33
            (b")",  vec![254, 255]),                // 33
            (b"*",  vec![249]),                     // 33
            (b"+",  vec![255, 127]),                // 33
            (b",",  vec![250]),                     // 33
            (b"-",  vec![91]),                      // 33
            (b".",  vec![95]),                      // 33
            (b"/",  vec![99]),                      // 33
            (b"0",  vec![7]),                       // 48
            (b"1",  vec![15]),                      // 49
            (b"2",  vec![23]),                      // 50
            (b"3",  vec![103]),                     // 51
            (b"4",  vec![107]),                     // 52
            (b"5",  vec![111]),                     // 53
            (b"6",  vec![115]),                     // 54
            (b"7",  vec![119]),                     // 55
            (b"8",  vec![123]),                     // 56
            (b"9",  vec![127]),                     // 57
            (b":",  vec![185]),                     // 58
            (b";",  vec![251]),                     // 59
            (b"<",  vec![255, 249]),                // 60
            (b"=",  vec![131]),                     // 61
            (b">",  vec![255, 191]),                // 62
            (b"?",  vec![255, 63]),                 // 63
            (b"@",  vec![255, 215]),                // 64
            (b"A",  vec![135]),                     // 65
            (b"B",  vec![187]),                     // 66
            (b"C",  vec![189]),                     // 67
            (b"D",  vec![191]),                     // 68
            (b"E",  vec![193]),                     // 69
            (b"F",  vec![195]),                     // 70
            (b"G",  vec![197]),                     // 71
            (b"H",  vec![199]),                     // 72
            (b"I",  vec![201]),                     // 73
            (b"J",  vec![203]),                     // 74
            (b"K",  vec![205]),                     // 75
            (b"L",  vec![207]),                     // 76
            (b"M",  vec![209]),                     // 77
            (b"N",  vec![211]),                     // 78
            (b"O",  vec![213]),                     // 79
            (b"P",  vec![215]),                     // 80
            (b"Q",  vec![217]),                     // 81
            (b"R",  vec![219]),                     // 82
            (b"S",  vec![221]),                     // 83
            (b"T",  vec![223]),                     // 84
            (b"U",  vec![225]),                     // 85
            (b"V",  vec![227]),                     // 86
            (b"W",  vec![229]),                     // 87
            (b"X",  vec![252]),                     // 88
            (b"Y",  vec![231]),                     // 89
            (b"Z",  vec![253]),                     // 90
            (b"[",  vec![255, 223]),                // 91
            (b"\\", vec![255, 254, 31]),            // 92
            (b"]",  vec![255, 231]),                // 93
            (b"^",  vec![255, 243]),                // 94
            (b"_",  vec![139]),                     // 95
            (b"`",  vec![255, 251]),                // 96
            (b"a",  vec![31]),                      // 97
            (b"b",  vec![143]),                     // 98
            (b"c",  vec![39]),                      // 99
            (b"d",  vec![147]),                     // 100
            (b"e",  vec![47]),                      // 101
            (b"f",  vec![151]),                     // 102
            (b"g",  vec![155]),                     // 103
            (b"h",  vec![159]),                     // 104
            (b"i",  vec![55]),                      // 105
            (b"j",  vec![233]),                     // 106
            (b"k",  vec![235]),                     // 107
            (b"l",  vec![163]),                     // 108
            (b"m",  vec![167]),                     // 109
            (b"n",  vec![171]),                     // 110
            (b"o",  vec![63]),                      // 111
            (b"p",  vec![175]),                     // 112
            (b"q",  vec![237]),                     // 113
            (b"r",  vec![179]),                     // 114
            (b"s",  vec![71]),                      // 115
            (b"t",  vec![79]),                      // 116
            (b"u",  vec![183]),                     // 117
            (b"v",  vec![239]),                     // 118
            (b"w",  vec![241]),                     // 119
            (b"x",  vec![243]),                     // 120
            (b"y",  vec![245]),                     // 121
            (b"z",  vec![247]),                     // 122
            (b"{",  vec![255, 253]),                // 123
            (b"|",  vec![255, 159]),                // 124
            (b"}",  vec![255, 247]),                // 125
            (b"~",  vec![255, 239]),                // 126
            (&[127],  vec![255, 255, 255, 207]),    // 127
            (&[128],  vec![255, 254, 111]),         // 128
            (&[129],  vec![255, 255, 75]),          // 129
            (&[130],  vec![255, 254, 127]),         // 130
            (&[131],  vec![255, 254, 143]),         // 131
            (&[132],  vec![255, 255, 79]),          // 132
            (&[133],  vec![255, 255, 83]),          // 133
            (&[134],  vec![255, 255, 87]),          // 134
            (&[135],  vec![255, 255, 179]),         // 135
            (&[136],  vec![255, 255, 91]),          // 136
            (&[137],  vec![255, 255, 181]),         // 137
            (&[138],  vec![255, 255, 183]),         // 138
            (&[139],  vec![255, 255, 185]),         // 139
            (&[140],  vec![255, 255, 187]),         // 140
            (&[141],  vec![255, 255, 189]),         // 141
            (&[142],  vec![255, 255, 235]),         // 142
            (&[143],  vec![255, 255, 191]),         // 143
            (&[144],  vec![255, 255, 236]),         // 144
            (&[145],  vec![255, 255, 237]),         // 145
            (&[146],  vec![255, 255, 95]),          // 146
            (&[147],  vec![255, 255, 193]),         // 147
            (&[148],  vec![255, 255, 238]),         // 148
            (&[149],  vec![255, 255, 195]),         // 149
            (&[150],  vec![255, 255, 197]),         // 150
            (&[151],  vec![255, 255, 199]),         // 151
            (&[152],  vec![255, 255, 201]),         // 152
            (&[153],  vec![255, 254, 231]),         // 153
            (&[154],  vec![255, 255, 99]),          // 154
            (&[155],  vec![255, 255, 203]),         // 155
            (&[156],  vec![255, 255, 103]),         // 156
            (&[157],  vec![255, 255, 205]),         // 157
            (&[158],  vec![255, 255, 207]),         // 158
            (&[159],  vec![255, 255, 239]),         // 159
            (&[160],  vec![255, 255, 107]),         // 160
            (&[161],  vec![255, 254, 239]),         // 161
            (&[162],  vec![255, 254, 159]),         // 162
            (&[163],  vec![255, 255, 111]),         // 163
            (&[164],  vec![255, 255, 115]),         // 164
            (&[165],  vec![255, 255, 209]),         // 165
            (&[166],  vec![255, 255, 211]),         // 166
            (&[167],  vec![255, 254, 247]),         // 167
            (&[168],  vec![255, 255, 213]),         // 168
            (&[169],  vec![255, 255, 119]),         // 169
            (&[170],  vec![255, 255, 123]),         // 170
            (&[171],  vec![255, 255, 240]),         // 171
            (&[172],  vec![255, 254, 255]),         // 172
            (&[173],  vec![255, 255, 127]),         // 173
            (&[174],  vec![255, 255, 215]),         // 174
            (&[175],  vec![255, 255, 217]),         // 175
            (&[176],  vec![255, 255, 7]),           // 176
            (&[177],  vec![255, 255, 15]),          // 177
            (&[178],  vec![255, 255, 131]),         // 178
            (&[179],  vec![255, 255, 23]),          // 179
            (&[180],  vec![255, 255, 219]),         // 180
            (&[181],  vec![255, 255, 135]),         // 181
            (&[182],  vec![255, 255, 221]),         // 182
            (&[183],  vec![255, 255, 223]),         // 183
            (&[184],  vec![255, 254, 175]),         // 184
            (&[185],  vec![255, 255, 139]),         // 185
            (&[186],  vec![255, 255, 143]),         // 186
            (&[187],  vec![255, 255, 147]),         // 187
            (&[188],  vec![255, 255, 225]),         // 188
            (&[189],  vec![255, 255, 151]),         // 189
            (&[190],  vec![255, 255, 155]),         // 190
            (&[191],  vec![255, 255, 227]),         // 191
            (&[192],  vec![255, 255, 248, 63]),     // 192
            (&[193],  vec![255, 255, 248, 127]),    // 193
            (&[194],  vec![255, 254, 191]),         // 194
            (&[195],  vec![255, 254, 63]),          // 195
            (&[196],  vec![255, 255, 159]),         // 196
            (&[197],  vec![255, 255, 229]),         // 197
            (&[198],  vec![255, 255, 163]),         // 198
            (&[199],  vec![255, 255, 246, 127]),    // 199
            (&[200],  vec![255, 255, 248, 191]),    // 200
            (&[201],  vec![255, 255, 248, 255]),    // 201
            (&[202],  vec![255, 255, 249, 63]),     // 202
            (&[203],  vec![255, 255, 251, 223]),    // 203
            (&[204],  vec![255, 255, 251, 255]),    // 204
            (&[205],  vec![255, 255, 249, 127]),    // 205
            (&[206],  vec![255, 255, 241]),         // 206
            (&[207],  vec![255, 255, 246, 255]),    // 207
            (&[208],  vec![255, 254, 95]),          // 208
            (&[209],  vec![255, 255, 31]),          // 209
            (&[210],  vec![255, 255, 249, 191]),    // 210
            (&[211],  vec![255, 255, 252, 31]),     // 211
            (&[212],  vec![255, 255, 252, 63]),     // 212
            (&[213],  vec![255, 255, 249, 255]),    // 213
            (&[214],  vec![255, 255, 252, 95]),     // 214
            (&[215],  vec![255, 255, 242]),         // 215
            (&[216],  vec![255, 255, 39]),          // 216
            (&[217],  vec![255, 255, 47]),          // 217
            (&[218],  vec![255, 255, 250, 63]),     // 218
            (&[219],  vec![255, 255, 250, 127]),    // 219
            (&[220],  vec![255, 255, 255, 223]),    // 220
            (&[221],  vec![255, 255, 252, 127]),    // 221
            (&[222],  vec![255, 255, 252, 159]),    // 222
            (&[223],  vec![255, 255, 252, 191]),    // 223
            (&[224],  vec![255, 254, 207]),         // 224
            (&[225],  vec![255, 255, 243]),         // 225
            (&[226],  vec![255, 254, 223]),         // 226
            (&[227],  vec![255, 255, 55]),          // 227
            (&[228],  vec![255, 255, 167]),         // 228
            (&[229],  vec![255, 255, 63]),          // 229
            (&[230],  vec![255, 255, 71]),          // 230
            (&[231],  vec![255, 255, 231]),         // 231
            (&[232],  vec![255, 255, 171]),         // 232
            (&[233],  vec![255, 255, 175]),         // 233
            (&[234],  vec![255, 255, 247, 127]),    // 234
            (&[235],  vec![255, 255, 247, 255]),    // 235
            (&[236],  vec![255, 255, 244]),         // 236
            (&[237],  vec![255, 255, 245]),         // 237
            (&[238],  vec![255, 255, 250, 191]),    // 238
            (&[239],  vec![255, 255, 233]),         // 239
            (&[240],  vec![255, 255, 250, 255]),    // 240
            (&[241],  vec![255, 255, 252, 223]),    // 241
            (&[242],  vec![255, 255, 251, 63]),     // 242
            (&[243],  vec![255, 255, 251, 127]),    // 243
            (&[244],  vec![255, 255, 252, 255]),    // 244
            (&[245],  vec![255, 255, 253, 31]),     // 245
            (&[246],  vec![255, 255, 253, 63]),     // 246
            (&[247],  vec![255, 255, 253, 95]),     // 247
            (&[248],  vec![255, 255, 253, 127]),    // 248
            (&[249],  vec![255, 255, 255, 239]),    // 249
            (&[250],  vec![255, 255, 253, 159]),    // 250
            (&[251],  vec![255, 255, 253, 191]),    // 251
            (&[252],  vec![255, 255, 253, 223]),    // 252
            (&[253],  vec![255, 255, 253, 255]),    // 253
            (&[254],  vec![255, 255, 254, 31]),     // 254
            (&[255],  vec![255, 255, 251, 191]),    // 255 
        ] // EOS is not a character
    }

    fn sample_literals() -> Vec<(Vec<u8>, Vec<u8>)> {
        vec![(
            vec![3, 4, 1, 2],
            vec![255, 255, 254, 63, 255, 255, 228, 255, 255, 177, 255, 255, 252, 95],
        ), (
            b":method".to_vec(),
            vec![185, 73, 83, 57, 228],
        ), (
            b":scheme".to_vec(),
            vec![184, 130, 78, 90, 75],
        ), (
            b":authority".to_vec(),
            vec![184, 59, 83, 57, 236, 50, 125, 127],
        ), (
            b"nibbstack.com".to_vec(),
            vec![168, 209, 198, 132, 140, 157, 87, 33, 233],
        ), (
            b"GET".to_vec(),
            vec![197, 131, 127],
        ), (
            b"http".to_vec(),
            vec![157, 41, 175],
        ), (
            b":path".to_vec(),
            vec![185, 88, 211, 63],
        ), (
            b"/images/top/sp2/cmn/logo-ns-130528.png".to_vec(),
            vec![96, 212, 142, 98, 161, 132, 158, 182, 17, 88, 152, 37, 53, 49,65, 230, 58, 213, 33, 96, 178, 6, 196, 242, 245, 213, 55],
        ), (
            b"hpack-test".to_vec(),
            vec![158, 177, 147, 170, 201, 42, 19],
        ), (
            b"xxxxxxx1".to_vec(),
            vec![243, 231, 207, 159, 62, 124, 135],
        ), (
            b"Mozilla/5.0 (Macintosh; Intel Mac OS X 10.8; rv:16.0) Gecko/20100101 Firefox/16.0".to_vec(),
            vec![208, 127, 102, 162, 129, 176, 218, 224, 83, 250, 208, 50, 26, 164, 157, 19, 253, 169, 146, 164, 150, 133, 52, 12, 138, 106, 220, 167, 226, 129, 2, 239, 125, 169, 103, 123, 129, 113, 112, 127, 106, 98, 41, 58, 157, 129, 0, 32, 0, 64, 21, 48, 154, 194, 202, 127, 44, 5, 197, 193],
        ), (
            b"accept".to_vec(),
            vec![25, 8, 90, 211],
        ), (
            b"Accept".to_vec(),
            vec![132, 132, 45, 105],
        ), (
            b"text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8".to_vec(),
            vec![73, 124, 165, 137, 211, 77, 31, 67, 174, 186, 12, 65, 164, 199, 169, 143, 51, 166, 154, 63, 223, 154, 104, 250, 29, 117, 208, 98, 13, 38, 61, 76, 121, 166, 143, 190, 208, 1, 119, 254, 190, 88, 249, 251, 237, 0, 23, 123],
        ), (
            b"cookie".to_vec(),
            vec![33, 207, 212, 197],
        ), (
            b"B=11231252zdf&b=3&s=0b".to_vec(),
            vec![187, 0, 66, 38, 66, 38, 197, 238, 73, 126, 35, 129, 159, 132, 64, 8, 255],
        ), (
            b"TE".to_vec(),
            vec![223, 131],
        ), (
            b"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Morbi non bibendum libero. Etiam ultrices lorem ut.".to_vec(),
            vec![206, 123, 11, 74, 134, 173, 22, 210, 164, 135, 160, 246, 40, 131, 37, 65, 210, 84, 253, 40, 67, 212, 130, 145, 37, 77, 182, 40, 57, 13, 89, 144, 67, 85, 50, 133, 160, 201, 93, 77, 7, 178, 51, 41, 81, 234, 82, 51, 70, 90, 164, 182, 149, 40, 52, 101, 176, 235, 169, 129, 38, 29, 42, 91, 66, 108, 49, 10, 133, 40, 61, 133, 165, 75, 82, 191],
        ), (
            b"!$%&A".to_vec(),
            vec![0b11111110, 0b00111111, 0b11110010, 0b10101111, 0b11000100, 0b00111111],
        ), (
            b"\0\0\0".to_vec(),
            vec![255, 199, 254, 63, 241],
        ), (
            b"\0\x01\x02\x03\x04\x05".to_vec(),
            vec![255, 199, 255, 253, 143, 255, 255, 226, 255, 255, 254, 63, 255, 255, 228, 255, 255, 254, 95],
        ), (
            b"\xFF\xF8".to_vec(),
            vec![255, 255, 251, 191, 255, 255, 95],
        )]
    }

    #[cfg(feature = "decode1")]
    #[test]
    fn decodes_characters_1bit() {
        for (data, code) in supported_characters().iter() {
            assert_eq!(*data, decode(&code, 1).unwrap());
        }
    }

    #[cfg(feature = "decode2")]
    #[test]
    fn decodes_characters_2bits() {
        for (data, code) in supported_characters().iter() {
            assert_eq!(*data, decode(&code, 2).unwrap());
        }
    }

    #[cfg(feature = "decode3")]
    #[test]
    fn decodes_characters_3bits() {
        for (data, code) in supported_characters().iter() {
            assert_eq!(*data, decode(&code, 3).unwrap());
        }
    }

    #[cfg(feature = "decode4")]
    #[test]
    fn decodes_characters_4bits() {
        for (data, code) in supported_characters().iter() {
            assert_eq!(*data, decode(&code, 4).unwrap());
        }
    }

    #[cfg(feature = "decode5")]
    #[test]
    fn decodes_characters_5bits() {
        for (data, code) in supported_characters().iter() {
            assert_eq!(*data, decode(&code, 5).unwrap());
        }
    }

    #[cfg(feature = "decode1")]
    #[test]
    fn decodes_literals_1bits() { 
        for (data, code) in sample_literals().iter() {
            assert_eq!(*data, decode(&code, 1).unwrap());
        }
    }

    #[cfg(feature = "decode2")]
    #[test]
    fn decodes_literals_2bits() { 
        for (data, code) in sample_literals().iter() {
            assert_eq!(*data, decode(&code, 2).unwrap());
        }
    }

    #[cfg(feature = "decode3")]
    #[test]
    fn decodes_literals_3bits() { 
        for (data, code) in sample_literals().iter() {
            assert_eq!(*data, decode(&code, 3).unwrap());
        }
    }

    #[cfg(feature = "decode4")]
    #[test]
    fn decodes_literals_4bits() { 
        for (data, code) in sample_literals().iter() {
            assert_eq!(*data, decode(&code, 4).unwrap());
        }
    }

    #[cfg(feature = "decode5")]
    #[test]
    fn decodes_literals_5bits() { 
        for (data, code) in sample_literals().iter() {
            assert_eq!(*data, decode(&code, 5).unwrap());
        }
    }
}
