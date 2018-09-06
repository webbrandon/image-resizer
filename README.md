# Image Resize Library Test 

I want to start evaluating the state of rust and its image processing libraries.  I do not expect many libraries to be very feature rich but can presume that they should be able to resize a image quickly with a variety of sampling algorithms. 

## Bench : Load To Resize Completion

```

| **Library** | **Original** | **Resized** | **Elapsed** |
|---|---|---|---|
| _raster_ | 2000x2000 | 700x700 | 4.671 |
| _raster_ | 2000x2000 | 150x150 | 3.630 |
| _image_ | 2000x2000 | 700x700 | 3.572 |
| _image_ | 2000x2000 | 150x150 | 3.245 |