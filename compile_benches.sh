g++ -DBENCH_MKL -fopenmp -std=c++20 -O3 lapack.cpp -ldiol -lmkl_rt -o ./target/mkl_bench
g++ -fopenmp -std=c++20 -O3 lapack.cpp "/usr/lib/x86_64-linux-gnu/openblas-pthread/liblapack.a" -ldiol -lopenblas -lgfortran -o ./target/openblas_bench
