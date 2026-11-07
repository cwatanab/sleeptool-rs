"""
Refined vector SVG icon generator for SleepTool with natural golden moon colors.
Optimized for high clarity at both 16x16, 32x32 and 256x256.
"""

import subprocess
import os

SVGS = {}

DEFS = """
  <defs>
    <!-- Natural Moon Gradients -->
    <!-- Natural gentle moon: warm cream gold to golden yellow -->
    <linearGradient id="moonGrad" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#FFF5B8" />
      <stop offset="30%" stop-color="#FDE047" />
      <stop offset="100%" stop-color="#EAB308" />
    </linearGradient>

    <!-- Natural sleeping / overcast moon for Pause (soft silvery moonlight gray) -->
    <linearGradient id="moonPauseGrad" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#E2E8F0" />
      <stop offset="50%" stop-color="#94A3B8" />
      <stop offset="100%" stop-color="#64748B" />
    </linearGradient>

    <!-- Overlay Badge Gradients -->
    <!-- Default: Midnight Sky Blue with bright white twinkling star (complements golden moon) -->
    <linearGradient id="badgeDefault" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#60A5FA" />
      <stop offset="100%" stop-color="#1D4ED8" />
    </linearGradient>

    <!-- CPU: Energetic Coral Red -->
    <linearGradient id="badgeCpu" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#F87171" />
      <stop offset="100%" stop-color="#DC2626" />
    </linearGradient>

    <!-- Network: Emerald Mint Green -->
    <linearGradient id="badgeNet" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#34D399" />
      <stop offset="100%" stop-color="#059669" />
    </linearGradient>

    <!-- Disk: Warm Terracotta / Orange Storage -->
    <linearGradient id="badgeDisk" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#FB923C" />
      <stop offset="100%" stop-color="#C2410C" />
    </linearGradient>

    <!-- Input: Playful Violet / Purple -->
    <linearGradient id="badgeInput" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#C084FC" />
      <stop offset="100%" stop-color="#7E22CE" />
    </linearGradient>

    <!-- Sound: Vibrant Melon Coral -->
    <linearGradient id="badgeSound" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#FB7185" />
      <stop offset="100%" stop-color="#E11D48" />
    </linearGradient>

    <!-- Process: Crisp Cyan Teal -->
    <linearGradient id="badgeProcess" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#38BDF8" />
      <stop offset="100%" stop-color="#0284C7" />
    </linearGradient>

    <!-- Pause: Striking Red -->
    <linearGradient id="badgePause" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#F87171" />
      <stop offset="100%" stop-color="#DC2626" />
    </linearGradient>

    <!-- White Badge Border with Drop Shadow for high separation -->
    <filter id="shadow" x="-30%" y="-30%" width="160%" height="160%">
      <feDropShadow dx="0" dy="5" stdDeviation="5" flood-color="#0F172A" flood-opacity="0.35" />
    </filter>
  </defs>
"""

def make_moon(paused=False):
    grad = "url(#moonPauseGrad)" if paused else "url(#moonGrad)"
    stroke_color = "#334155" if paused else "#92400E"
    face_color = "#334155" if paused else "#78350F"
    cheek_fill = "#94A3B8" if paused else "#FB7185"
    bubble_color = "#FFFFFF" if paused else "#FFFBEB"

    return f"""
    <!-- Moon Body (plump, rounded, smiling sleeping crescent in natural moon golden hue) -->
    <path d="M 142 20
             C 68 20 18 76 18 148
             C 18 208 68 242 128 242
             C 142 242 156 238 164 233
             C 114 218 80 176 80 128
             C 80 72 114 34 154 23
             C 150 21 146 20 142 20 Z"
          fill="{grad}"
          stroke="{stroke_color}"
          stroke-width="8"
          stroke-linejoin="round"
          stroke-linecap="round" />

    <!-- Cute little star pom-pom on moon tip -->
    <circle cx="142" cy="20" r="9" fill="#FFFBEB" stroke="{stroke_color}" stroke-width="4" />

    <!-- Closed smiling sleeping eye -->
    <path d="M 42 128 C 47 140 63 140 68 128"
          fill="none" stroke="{face_color}" stroke-width="7" stroke-linecap="round" />
    <!-- Cute eyelash -->
    <line x1="55" y1="138" x2="55" y2="145" stroke="{face_color}" stroke-width="5" stroke-linecap="round" />

    <!-- Kawaii blush cheek -->
    <ellipse cx="42" cy="148" rx="10" ry="6" fill="{cheek_fill}" opacity="0.9" />

    <!-- Sweet smile -->
    <path d="M 66 150 C 72 157 80 155 84 148"
          fill="none" stroke="{face_color}" stroke-width="5" stroke-linecap="round" />

    <!-- Dream bubbles if active -->
    {f'''
    <circle cx="94" cy="74" r="6" fill="{bubble_color}" opacity="0.85" />
    <circle cx="110" cy="52" r="8" fill="{bubble_color}" opacity="0.95" />
    ''' if not paused else ''}
    """

def wrap_svg(content):
    return f"""<svg xmlns="http://www.w3.org/2000/svg" width="256" height="256" viewBox="0 0 256 256">
{DEFS}
{content}
</svg>"""

# 1. Default: Friendly glowing Star on Midnight Blue Badge (Natural Moon + Night Sky Star)
SVGS["Default"] = wrap_svg(f"""
{make_moon(paused=False)}
  <g filter="url(#shadow)">
    <!-- White backing & colored badge -->
    <circle cx="182" cy="168" r="56" fill="url(#badgeDefault)" stroke="#FFFFFF" stroke-width="8" />
    <!-- 4-point chubby sparkle star in bright cream white -->
    <path d="M 182 126
             Q 182 168 222 168
             Q 182 168 182 210
             Q 182 168 142 168
             Q 182 168 182 126 Z"
          fill="#FFFFFF" />
    <!-- Cute sparkle dots -->
    <circle cx="212" cy="138" r="6" fill="#FFFFFF" />
    <circle cx="152" cy="198" r="4.5" fill="#FFFFFF" />
  </g>
""")

# 2. CPU: Chubby Microchip with bold lightning bolt
SVGS["CPU"] = wrap_svg(f"""
{make_moon(paused=False)}
  <g filter="url(#shadow)">
    <circle cx="182" cy="168" r="56" fill="url(#badgeCpu)" stroke="#FFFFFF" stroke-width="8" />

    <!-- Chubby chip pins -->
    <!-- Top & bottom -->
    <line x1="168" y1="126" x2="168" y2="136" stroke="#FFFFFF" stroke-width="8" stroke-linecap="round" />
    <line x1="182" y1="126" x2="182" y2="136" stroke="#FFFFFF" stroke-width="8" stroke-linecap="round" />
    <line x1="196" y1="126" x2="196" y2="136" stroke="#FFFFFF" stroke-width="8" stroke-linecap="round" />
    <line x1="168" y1="200" x2="168" y2="210" stroke="#FFFFFF" stroke-width="8" stroke-linecap="round" />
    <line x1="182" y1="200" x2="182" y2="210" stroke="#FFFFFF" stroke-width="8" stroke-linecap="round" />
    <line x1="196" y1="200" x2="196" y2="210" stroke="#FFFFFF" stroke-width="8" stroke-linecap="round" />
    <!-- Left & right -->
    <line x1="140" y1="154" x2="150" y2="154" stroke="#FFFFFF" stroke-width="8" stroke-linecap="round" />
    <line x1="140" y1="168" x2="150" y2="168" stroke="#FFFFFF" stroke-width="8" stroke-linecap="round" />
    <line x1="140" y1="182" x2="150" y2="182" stroke="#FFFFFF" stroke-width="8" stroke-linecap="round" />
    <line x1="214" y1="154" x2="224" y2="154" stroke="#FFFFFF" stroke-width="8" stroke-linecap="round" />
    <line x1="214" y1="168" x2="224" y2="168" stroke="#FFFFFF" stroke-width="8" stroke-linecap="round" />
    <line x1="214" y1="182" x2="224" y2="182" stroke="#FFFFFF" stroke-width="8" stroke-linecap="round" />

    <!-- Chip Body -->
    <rect x="150" y="136" width="64" height="64" rx="14" fill="#FFFFFF" />

    <!-- Bold Cute Lightning Bolt -->
    <path d="M 186 144 L 168 168 L 180 168 L 176 192 L 196 166 L 184 166 Z"
          fill="#DC2626" />
  </g>
""")

# 3. Network: Bold, high-contrast Up/Down traffic arrows
SVGS["Network"] = wrap_svg(f"""
{make_moon(paused=False)}
  <g filter="url(#shadow)">
    <circle cx="182" cy="168" r="56" fill="url(#badgeNet)" stroke="#FFFFFF" stroke-width="8" />

    <!-- Upload Arrow (Left, pointing UP, bold) -->
    <line x1="164" y1="196" x2="164" y2="150" stroke="#FFFFFF" stroke-width="11" stroke-linecap="round" />
    <path d="M 148 164 L 164 142 L 180 164"
          fill="none" stroke="#FFFFFF" stroke-width="10" stroke-linecap="round" stroke-linejoin="round" />

    <!-- Download Arrow (Right, pointing DOWN, bold) -->
    <line x1="200" y1="142" x2="200" y2="188" stroke="#FFFFFF" stroke-width="11" stroke-linecap="round" />
    <path d="M 184 174 L 200 196 L 216 174"
          fill="none" stroke="#FFFFFF" stroke-width="10" stroke-linecap="round" stroke-linejoin="round" />
  </g>
""")

# 4. Disk: Cute Floppy Disk with label and shutter
SVGS["Disk"] = wrap_svg(f"""
{make_moon(paused=False)}
  <g filter="url(#shadow)">
    <circle cx="182" cy="168" r="56" fill="url(#badgeDisk)" stroke="#FFFFFF" stroke-width="8" />

    <!-- Floppy Outer Card -->
    <rect x="152" y="136" width="60" height="64" rx="10" fill="#FFFFFF" />
    <!-- Top Shutter (Deep Terracotta) -->
    <rect x="164" y="136" width="32" height="24" rx="4" fill="#C2410C" />
    <rect x="171" y="140" width="7" height="14" rx="2" fill="#FFFFFF" />
    <!-- Bottom Label Area with two bold write stripes -->
    <rect x="160" y="168" width="44" height="24" rx="5" fill="#FFEDD5" />
    <line x1="168" y1="175" x2="196" y2="175" stroke="#C2410C" stroke-width="4.5" stroke-linecap="round" />
    <line x1="168" y1="184" x2="190" y2="184" stroke="#C2410C" stroke-width="4.5" stroke-linecap="round" />
  </g>
""")

# 5. Input: Cute Gamepad / Controller
SVGS["Input"] = wrap_svg(f"""
{make_moon(paused=False)}
  <g filter="url(#shadow)">
    <circle cx="182" cy="168" r="56" fill="url(#badgeInput)" stroke="#FFFFFF" stroke-width="8" />

    <!-- Controller Body -->
    <rect x="150" y="144" width="64" height="46" rx="20" fill="#FFFFFF" />
    <!-- D-Pad (Left) -->
    <path d="M 160 167 L 172 167 M 166 161 L 166 173"
          stroke="#7E22CE" stroke-width="6" stroke-linecap="round" />
    <!-- Action buttons (Right) -->
    <circle cx="194" cy="163" r="4.5" fill="#EC4899" />
    <circle cx="204" cy="171" r="4.5" fill="#3B82F6" />
    <!-- Cute smile arc at bottom center -->
    <path d="M 179 179 Q 182 183 185 179"
          fill="none" stroke="#7E22CE" stroke-width="4" stroke-linecap="round" />
  </g>
""")

# 6. Sound: Cheerful Speaker with expanding sound waves
SVGS["Sound"] = wrap_svg(f"""
{make_moon(paused=False)}
  <g filter="url(#shadow)">
    <circle cx="182" cy="168" r="56" fill="url(#badgeSound)" stroke="#FFFFFF" stroke-width="8" />

    <!-- Speaker Body -->
    <rect x="153" y="156" width="16" height="24" rx="5" fill="#FFFFFF" />
    <!-- Speaker Cone -->
    <path d="M 168 156 L 186 142 L 186 194 L 168 180 Z" fill="#FFFFFF" />
    <!-- Sound Waves (thick bold arcs for 16px visibility) -->
    <path d="M 195 152 C 201 158 201 178 195 184"
          fill="none" stroke="#FFFFFF" stroke-width="7" stroke-linecap="round" />
    <path d="M 206 142 C 218 154 218 182 206 194"
          fill="none" stroke="#FFFFFF" stroke-width="7" stroke-linecap="round" />
  </g>
""")

# 7. Process: Cute 6-Tooth Cogwheel with large center hole
SVGS["Process"] = wrap_svg(f"""
{make_moon(paused=False)}
  <g filter="url(#shadow)">
    <circle cx="182" cy="168" r="56" fill="url(#badgeProcess)" stroke="#FFFFFF" stroke-width="8" />

    <!-- 6-Tooth Smooth Bold Gear -->
    <g transform="translate(182, 168)">
      <rect x="-9" y="-36" width="18" height="72" rx="7" fill="#FFFFFF" />
      <rect x="-9" y="-36" width="18" height="72" rx="7" fill="#FFFFFF" transform="rotate(60)" />
      <rect x="-9" y="-36" width="18" height="72" rx="7" fill="#FFFFFF" transform="rotate(120)" />
      <!-- Center disc -->
      <circle cx="0" cy="0" r="26" fill="#FFFFFF" />
      <!-- Inner hole (badge colored) -->
      <circle cx="0" cy="0" r="12" fill="#0284C7" />
    </g>
  </g>
""")

# 8. Pause: Striking Red Pause Bars on Sleepy Silvery Moon
SVGS["Pause"] = wrap_svg(f"""
{make_moon(paused=True)}
  <g filter="url(#shadow)">
    <circle cx="182" cy="168" r="56" fill="url(#badgePause)" stroke="#FFFFFF" stroke-width="8" />

    <!-- Bold Rounded Pause Bars -->
    <rect x="164" y="144" width="14" height="48" rx="7" fill="#FFFFFF" />
    <rect x="188" y="144" width="14" height="48" rx="7" fill="#FFFFFF" />
  </g>
""")

def main():
    output_dir = "/workspaces/SleepTool/assets"
    for name, svg in SVGS.items():
        svg_path = f"/tmp/{name}.svg"
        png_path = f"{output_dir}/{name}.png"
        with open(svg_path, "w", encoding="utf-8") as f:
            f.write(svg)

        cmd = [
            "ffmpeg", "-y",
            "-i", svg_path,
            "-update", "1",
            "-frames:v", "1",
            png_path
        ]
        res = subprocess.run(cmd, capture_output=True)
        if res.returncode != 0:
            print(f"Error {name}: {res.stderr.decode('utf-8')}")
        else:
            print(f"Generated {png_path}")

if __name__ == "__main__":
    main()
