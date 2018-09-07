# Image Resize Library Test 

I want to start evaluating the state of rust and its image processing libraries.  I do not expect many libraries to be very feature rich but can presume that they should be able to resize a image quickly with a variety of sampling algorithms. 

## Benchmark : Resize Functions

| **Library** | **Original** | **Resized** | **Sampling** |  **Function** | **Elapsed** |
|---|---|---|---|---|---|
| _raster_ | 2000x2000 | 150x150 | Unknown | resize | 0.185 |
| _image_ | 2000x2000 | 150x150 | Unknown | thumbnail | 0.996 |
| _image_ | 2000x2000 | 150x150 | Lanczos3 | resize | 8.182 |
| _image_ | 2000x2000 | 150x150 | Gaussian | resize | 7.069 |
| _image_ | 2000x2000 | 150x150 | CatmullRom | resize | 4.683 |
| _image_ | 2000x2000 | 150x150 | Nearest | resize | 0.204 |
| _image_ | 2000x2000 | 150x150 | Triangle | resize | 2.274 |

Test performed with :
```
  MacBookPro11,5
  Processor Name:	Intel Core i7
  Processor Speed:	2.8 GHz
  Number of Processors:	1
  Total Number of Cores:	4
  L2 Cache (per Core):	256 KB
  L3 Cache:	6 MB
  Memory:	16 GB
```