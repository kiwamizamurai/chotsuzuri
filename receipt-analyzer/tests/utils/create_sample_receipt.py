from PIL import Image, ImageDraw, ImageFont
import os

WIDTH = 800
HEIGHT = 600

SCRIPT_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SAMPLES_DIR = os.path.join(SCRIPT_DIR, 'samples')

image = Image.new('RGB', (WIDTH, HEIGHT), 'white')
draw = ImageDraw.Draw(image)

try:
    font = ImageFont.truetype('/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc', 30)
except:
    font = ImageFont.load_default()

text_lines = [
    "領 収 書",
    "2024年3月20日",
    "株式会社ボッタクリサセル様",
    "￥3,300-",
    "",
    "内訳：",
    "文房具        ￥2,200",
    "コピー用紙     ￥1,100",
    "",
    "上記正に領収いたしました",
    "",
    "株式会社サンプル商店",
    "東京都千代田区1-2-3",
]

y = 50
for line in text_lines:
    text_width = draw.textlength(line, font=font)
    x = (WIDTH - text_width) / 2
    draw.text((x, y), line, fill='black', font=font)
    y += 40

os.makedirs(SAMPLES_DIR, exist_ok=True)
sample_path = os.path.join(SAMPLES_DIR, 'sample_receipt.png')
image.save(sample_path)
print(f"saved image: {sample_path}")
