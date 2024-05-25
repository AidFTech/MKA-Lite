import MKA

if __name__ == '__main__':
    mka = MKA.MKA(False, False)
    try:
        while mka.run:
            mka.loop()
    except KeyboardInterrupt:
        mka.stop()
        sys.exit(0)
