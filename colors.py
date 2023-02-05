import colorsys

def distinct_colors(n):
    huePartition = 1.0 / (n + 1)
    return (colorsys.hsv_to_rgb(huePartition * value, 1.0, 1.0) for value in range(0, n))
