# Image Resize Library Test 

I want to start evaluating the state of rust and its image processing libraries.  I do not expect many libraries to be very feature rich but can presume that they should be able to resize a image quickly with a variety of sampling algorithms. 

**Purpose**  
Goal is to see if migrating our image resizing app using  [sharp](http://sharp.pixelplumbing.com/en/stable/) image library. Sharp can perform the same conversion of a 2000x2000 to an 150x150 image size in _0.101s_. If we can beat this metric we have a purpose to investigate migrating further.

## Build
Test must be built as a release for speed optimizations.

```bash
cargo build --release
cargo install
```

## Test
```bash
cd test
./benchmark-72ppi.
```

## Benchmark : Resize Functions
| **Library** | **Original** | **Resized** | **Sampling** |  **Function** | **Elapsed** |
|---|---|---|---|---|---|
| _raster_ | 2000x2000 | 150x150 | Unknown | resize | 0.006 |
| _image_ | 2000x2000 | 150x150 | Unknown | thumbnail | 0.013 |
| _image_ | 2000x2000 | 150x150 | Lanczos3 | resize | 0.607 |
| _image_ | 2000x2000 | 150x150 | Gaussian | resize | 0.342 |
| _image_ | 2000x2000 | 150x150 | CatmullRom | resize | 0.138 |
| _image_ | 2000x2000 | 150x150 | Nearest | resize | 0.009 |
| _image_ | 2000x2000 | 150x150 | Triangle | resize | 0.070 |

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

## Status 
We have beaten sharp!!!

Need to check/verify quality output of images.
