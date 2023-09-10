# DetectPicSteganography
Rust tool that detects potential tampering in image files using Rascii

Takes in two files just like diff does. These files are images you want to check for tampering.

After checking for tampering, it writes the images to a file that uses unicode block characters to
highlight added information. Tailor this tool to your liking. Uses a provided simple diff tool but
you can simply point it to GNU.

From: m0nZSt3r and $t@$h, QVLx Labs

![image](https://github.com/STashakkori/DetectPicSteganography/assets/4257899/817449df-5b3b-48c5-9b2d-287438968b40)
