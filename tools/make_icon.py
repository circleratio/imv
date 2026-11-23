"""Generate the master application icon (app-icon.png, 1024x1024).

Usage:
    python tools/make_icon.py [output_path]
    npm run tauri icon app-icon.png   # regenerate src-tauri/icons from it
"""

import sys

from PIL import Image, ImageDraw, ImageFilter

SIZE = 1024
SCALE = 4  # draw at 4x and downsample for smooth edges
S = SIZE * SCALE


def lerp(a, b, t):
    return tuple(round(x + (y - x) * t) for x, y in zip(a, b))


def vertical_gradient(w, h, top, bottom):
    grad = Image.new("RGBA", (1, h))
    for y in range(h):
        grad.putpixel((0, y), lerp(top, bottom, y / max(h - 1, 1)) + (255,))
    return grad.resize((w, h))


def diagonal_gradient(w, h, start, end):
    # Blend a horizontal and vertical ramp to get a top-left -> bottom-right gradient
    small = Image.new("RGBA", (64, 64))
    for y in range(64):
        for x in range(64):
            small.putpixel((x, y), lerp(start, end, (x + y) / 126) + (255,))
    return small.resize((w, h), Image.BILINEAR)


def rounded_mask(w, h, radius):
    mask = Image.new("L", (w, h), 0)
    ImageDraw.Draw(mask).rounded_rectangle((0, 0, w - 1, h - 1), radius=radius, fill=255)
    return mask


def make_photo_card(w, h, with_picture):
    """A white photo print with an optional landscape picture inside."""
    border = int(w * 0.07)
    radius = int(w * 0.05)
    card = Image.new("RGBA", (w, h), (0, 0, 0, 0))
    ImageDraw.Draw(card).rounded_rectangle(
        (0, 0, w - 1, h - 1), radius=radius, fill=(250, 251, 255, 255)
    )
    if not with_picture:
        return card

    pw, ph = w - border * 2, h - border * 2
    pic = vertical_gradient(pw, ph, (120, 200, 250), (205, 235, 255))
    d = ImageDraw.Draw(pic)

    # Sun
    r = int(pw * 0.11)
    cx, cy = int(pw * 0.72), int(ph * 0.28)
    d.ellipse((cx - r, cy - r, cx + r, cy + r), fill=(255, 196, 61, 255))

    # Back mountain range
    d.polygon(
        [(0, ph * 0.78), (pw * 0.30, ph * 0.42), (pw * 0.52, ph * 0.66),
         (pw * 0.70, ph * 0.50), (pw, ph * 0.80), (pw, ph), (0, ph)],
        fill=(72, 170, 160, 255),
    )
    # Front hill
    d.polygon(
        [(0, ph * 0.88), (pw * 0.25, ph * 0.70), (pw * 0.55, ph * 0.86),
         (pw * 0.80, ph * 0.74), (pw, ph * 0.86), (pw, ph), (0, ph)],
        fill=(38, 128, 120, 255),
    )

    pic_mask = rounded_mask(pw, ph, int(radius * 0.6))
    card.paste(pic, (border, border), pic_mask)
    return card


def paste_rotated(base, layer, center, angle, shadow=True):
    rotated = layer.rotate(angle, resample=Image.BICUBIC, expand=True)
    x = center[0] - rotated.width // 2
    y = center[1] - rotated.height // 2
    if shadow:
        alpha = rotated.getchannel("A")
        sh = Image.new("RGBA", rotated.size, (8, 20, 60, 0))
        sh.putalpha(alpha.point(lambda a: int(a * 0.45)))
        sh = sh.filter(ImageFilter.GaussianBlur(S * 0.012))
        base.alpha_composite(sh, (x + int(S * 0.006), y + int(S * 0.016)))
    base.alpha_composite(rotated, (x, y))


def main(out_path):
    canvas = Image.new("RGBA", (S, S), (0, 0, 0, 0))

    # Rounded-square background
    margin = int(S * 0.04)
    bw = S - margin * 2
    bg = diagonal_gradient(bw, bw, (64, 120, 240), (28, 44, 120))
    canvas.paste(bg, (margin, margin), rounded_mask(bw, bw, int(bw * 0.22)))

    # Stacked photo prints: the back one hints at "the next random image"
    cw, ch = int(S * 0.54), int(S * 0.46)
    center = (S // 2, int(S * 0.52))
    back = make_photo_card(cw, ch, with_picture=False)
    back_tinted = Image.new("RGBA", back.size, (190, 205, 240, 0))
    back_tinted.putalpha(back.getchannel("A"))
    paste_rotated(canvas, back_tinted, (center[0] - int(S * 0.03), center[1] - int(S * 0.04)), 14)

    front = make_photo_card(cw, ch, with_picture=True)
    paste_rotated(canvas, front, (center[0] + int(S * 0.02), center[1] + int(S * 0.03)), -6)

    canvas.resize((SIZE, SIZE), Image.LANCZOS).save(out_path)


if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else "app-icon.png")
