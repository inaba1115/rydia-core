def test_import():
    import rydia


def test_sinosc_smoke():
    import rydia

    osc = rydia.SinOsc(48_000.0)
    y = osc.process(440.0)

    assert isinstance(y, float)


def test_sinosc_reset_smoke():
    import rydia

    osc = rydia.SinOsc(48_000.0)
    osc.process(440.0)
    osc.reset()

    y = osc.process(440.0)
    assert isinstance(y, float)


def test_white_noise_smoke():
    import rydia

    noise = rydia.WhiteNoise()
    x = noise.process()

    assert isinstance(x, float)
