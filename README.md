# Image Resize Library Test 

I want to start evaluating the state of rust and its image processing libraries.  I do not expect many libraries to be very feature rich but can presume that they should be able to resize a image quickly with a variety of sampling algorithms. 

## Bench : Load To Resize Completion


| **Library** | **Original** | **Resized** | **Sampling** |  **Function** | **Elapsed** |
|---|---|---|---|---|---|
| _raster_ | 2000x2000 | 700x700 | Unknown | resize | 4.671 |
| _raster_ | 2000x2000 | 150x150 | Unknown | resize | 3.630 |
| _image_ | 2000x2000 | 700x700 | Unknown | thumbnail | 3.572 |
| _image_ | 2000x2000 | 150x150 | Unknown | thumbnail | 3.245 |
| _image_ | 2000x2000 | 150x150 | Lanczos3 | resize | 10.593 |
| _image_ | 2000x2000 | 150x150 | Gaussian | resize | 9.270 |
| _image_ | 2000x2000 | 150x150 | CatmullRom | resize | 6.910 |
| _image_ | 2000x2000 | 150x150 | Nearest | resize | 2.458 |
| _image_ | 2000x2000 | 150x150 | Triangle | resize | 4.549 |

Test performed with :
```
  Processor Name:	Intel Core i7
  Processor Speed:	2.8 GHz
  Number of Processors:	1
  Total Number of Cores:	4
  L2 Cache (per Core):	256 KB
  L3 Cache:	6 MB
  Memory:	16 GB
```