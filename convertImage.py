from PIL import Image
import math

im = Image.open('test.png') # Can be many different formats.
pix = im.convert('L')
file = open("test.txt","w+")
for y in range(im.height):
    for x in range(im.width):
        file.write(str(round(pix.getpixel((x,y))/255)))
file.close()
