# Encoding

- First pixel is the number of values valid on the last chunk for exemple if the last chunk is  [12,34] the first pixel value would be [2,2,2]
- Second pixel is the full height of the valid pixels on the last page so max is MAX_HEIGHT
- Third pixel is the Number of pixels valid on the last line the format being "85ffff" = 85 pixels

- The pixel number 4 is the number of char on the filename and the valid char on the last pixels before the data exemple:
    - "file.pdf" => "66696c652e706466" => split by 6 => ["66696c","652e70","6466"] => last pixel = (8,0,4) => like (filename.len(), 0, lastColorDigitValidChars) 
    - So we know that ( `8` * 2 = 16) and 16 % 6 = `4` => 4 is the lastColorDigitValidChars
    - On the decoder, we will be able to know that the next (`8` * 2 = 16) => 16 / 6 = 2.6666 => ceil(2.6666) => 3 so the next 3 pixels are the filename

- The filename.len() must be < 255 the filename will be cropped from the start to avoid losing the extension