gcc ./Backend/*.c -o ./MKA_Lite $(python3-config --embed --ldflags) -lpigpio -lrt
