import os
from PIL import Image, ImageDraw

def save_rgba_and_png(name, img):
    os.makedirs("assets/icons", exist_ok=True)
    # Save preview PNG
    img.save(f"assets/icons/{name}.png")
    # Save raw RGBA bytes
    raw = img.tobytes("raw", "RGBA")
    with open(f"assets/icons/{name}.rgba", "wb") as f:
        f.write(raw)

def create_base_img():
    return Image.new("RGBA", (32, 32), (0, 0, 0, 0))

# 1. Default (Active Coffee Cup)
img = create_base_img()
draw = ImageDraw.Draw(img)
# Draw cup body
draw.rounded_rectangle([6, 12, 22, 26], radius=4, outline=(255, 204, 0, 255), width=2)
# Draw handle
draw.arc([18, 14, 26, 22], start=-90, end=90, fill=(255, 204, 0, 255), width=2)
# Draw steam
draw.arc([10, 4, 14, 10], start=0, end=180, fill=(255, 204, 0, 200), width=1)
draw.arc([14, 4, 18, 10], start=180, end=360, fill=(255, 204, 0, 200), width=1)
save_rgba_and_png("default", img)

# 2. Paused (Muted Coffee Cup with Slash)
img = create_base_img()
draw = ImageDraw.Draw(img)
# Draw cup body (gray)
draw.rounded_rectangle([6, 12, 22, 26], radius=4, outline=(140, 140, 140, 255), width=2)
# Draw handle (gray)
draw.arc([18, 14, 26, 22], start=-90, end=90, fill=(140, 140, 140, 255), width=2)
# Draw diagonal pause/slash line (reddish-gray)
draw.line([4, 28, 28, 4], fill=(200, 80, 80, 255), width=2)
save_rgba_and_png("paused", img)

# 3. CPU (Microchip)
img = create_base_img()
draw = ImageDraw.Draw(img)
# Center square
draw.rectangle([8, 8, 24, 24], outline=(255, 51, 102, 255), width=2)
# Inner core
draw.rectangle([12, 12, 20, 20], fill=(255, 51, 102, 120))
# Pins
for i in [11, 15, 19, 23]:
    # Top/Bottom
    draw.line([i, 4, i, 7], fill=(255, 51, 102, 255), width=1)
    draw.line([i, 25, i, 28], fill=(255, 51, 102, 255), width=1)
    # Left/Right
    draw.line([4, i, 7, i], fill=(255, 51, 102, 255), width=1)
    draw.line([25, i, 28, i], fill=(255, 51, 102, 255), width=1)
save_rgba_and_png("cpu", img)

# 4. Network (Wi-Fi Signals)
img = create_base_img()
draw = ImageDraw.Draw(img)
# Dot at bottom-left
draw.ellipse([6, 24, 8, 26], fill=(51, 153, 255, 255))
# Inner wave
draw.arc([4, 14, 18, 28], start=270, end=360, fill=(51, 153, 255, 255), width=2)
# Outer wave
draw.arc([0, 6, 26, 32], start=270, end=360, fill=(51, 153, 255, 255), width=2)
save_rgba_and_png("network", img)

# 5. Disk (Cylinder Hard Drive)
img = create_base_img()
draw = ImageDraw.Draw(img)
# Top face
draw.ellipse([8, 6, 24, 12], outline=(255, 204, 0, 255), width=2)
# Middle face
draw.arc([8, 12, 24, 18], start=0, end=180, fill=(255, 204, 0, 255), width=2)
# Bottom face
draw.arc([8, 18, 24, 24], start=0, end=180, fill=(255, 204, 0, 255), width=2)
# Vertical lines
draw.line([8, 9, 8, 21], fill=(255, 204, 0, 255), width=2)
draw.line([24, 9, 24, 21], fill=(255, 204, 0, 255), width=2)
save_rgba_and_png("disk", img)

# 6. Sound (Speaker with Waves)
img = create_base_img()
draw = ImageDraw.Draw(img)
# Speaker body
draw.polygon([(6, 12), (12, 12), (18, 6), (18, 26), (12, 20), (6, 20)], fill=(51, 255, 153, 255))
# Sound waves
draw.arc([10, 8, 24, 24], start=-60, end=60, fill=(51, 255, 153, 255), width=2)
draw.arc([6, 2, 28, 30], start=-60, end=60, fill=(51, 255, 153, 200), width=2)
save_rgba_and_png("sound", img)

# 7. Process (Dynamic Ring Arrow)
img = create_base_img()
draw = ImageDraw.Draw(img)
# Circular path
draw.arc([6, 6, 26, 26], start=0, end=270, fill=(204, 102, 255, 255), width=3)
# Arrow head
draw.polygon([(26, 10), (22, 18), (30, 18)], fill=(204, 102, 255, 255))
save_rgba_and_png("process", img)

# 8. Printer
img = create_base_img()
draw = ImageDraw.Draw(img)
# Body
draw.rounded_rectangle([6, 12, 26, 24], radius=2, outline=(255, 153, 51, 255), width=2)
# Paper feeding top
draw.rectangle([10, 6, 22, 12], outline=(255, 153, 51, 255), width=1)
# Paper sheet bottom
draw.rectangle([10, 22, 22, 28], fill=(255, 153, 51, 180))
save_rgba_and_png("printer", img)

# 9. App ICO (high-res vector rendering)
def create_high_res_icon():
    img = Image.new("RGBA", (256, 256), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    # Draw cup body
    draw.rounded_rectangle([48, 96, 176, 208], radius=32, outline=(255, 204, 0, 255), width=16)
    # Draw handle
    draw.arc([144, 112, 208, 176], start=-90, end=90, fill=(255, 204, 0, 255), width=16)
    # Draw steam
    draw.arc([80, 32, 112, 80], start=0, end=180, fill=(255, 204, 0, 200), width=8)
    draw.arc([112, 32, 144, 80], start=180, end=360, fill=(255, 204, 0, 200), width=8)
    return img

high_res = create_high_res_icon()
sizes = [16, 32, 48, 64, 128, 256]
icon_imgs = []
for sz in sizes:
    icon_imgs.append(high_res.resize((sz, sz), Image.Resampling.LANCZOS))
ico_path = "assets/icons/app.ico"
icon_imgs[0].save(ico_path, format="ICO", append_images=icon_imgs[1:])

print("Beautiful system tray icons and app.ico generated successfully in assets/icons/!")
