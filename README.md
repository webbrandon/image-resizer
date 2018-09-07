# Image Resize Library Test 

I want to start evaluating the state of rust and its image processing libraries.  I do not expect many libraries to be very feature rich but can presume that they should be able to resize a image quickly with a variety of sampling algorithms. 

## Benchmark : Resize Functions

Must be built as a release for speed optimizations.
- ```cargo build --release && cargo install```

| **Library** | **Original** | **Resized** | **Sampling** |  **Function** | **Elapsed** |
|---|---|---|---|---|---|
| _raster_ | 2000x2000 | 150x150 | Unknown | resize | 0.006 |
| _image_ | 2000x2000 | 150x150 | Unknown | thumbnail | 0.013 |
| _image_ | 2000x2000 | 150x150 | Lanczos3 | resize | 1.281 |
| _image_ | 2000x2000 | 150x150 | Gaussian | resize | 0.342 |
| _image_ | 2000x2000 | 150x150 | CatmullRom | resize | 0.309 |
| _image_ | 2000x2000 | 150x150 | Nearest | resize | 0.020 |
| _image_ | 2000x2000 | 150x150 | Triangle | resize | 0.072 |

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

## Purpose 

Goal is to migrate image resizing app from Node.js using [sharp](http://sharp.pixelplumbing.com/en/stable/).  The resizing for this library with equivalent image test for below metrics is _0.101s_.  If I can find a faster path to resizing I can begin moving forward on this initiative.

## Status 
We have beaten Sharp.js!!!
