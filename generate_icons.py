import os


def save_rgba(name, color):
    """Generate a 32x32 RGBA raw image file."""
    os.makedirs("assets/icons", exist_ok=True)
    width, height = 32, 32
    raw = bytes(color) * (width * height)
    with open(f"assets/icons/{name}.rgba", "wb") as f:
        f.write(raw)


icons = {
    "default": [128, 128, 128, 255],
    "cpu": [255, 0, 0, 255],
    "network": [0, 128, 255, 255],
    "disk": [255, 255, 0, 255],
    "sound": [0, 255, 0, 255],
    "process": [255, 0, 255, 255],
    "printer": [255, 128, 0, 255],
    "paused": [192, 192, 192, 255],
}

for name, color in icons.items():
    save_rgba(name, color)

print("RGBA icons generated in assets/icons/")
