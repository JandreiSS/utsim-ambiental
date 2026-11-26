# Desenha o ícone do UTSim Ambiental: folha sobre engrenagem, estilo plano da família.
import math, sys
from PIL import Image, ImageDraw
out = sys.argv[1]
S = 1024; k = S/256
img = Image.new("RGBA", (S, S), (0,0,0,0))
# fundo com gradiente vertical, cantos arredondados (rx 48)
top = (0x5c,0xc8,0x6e); bot = (0x1b,0x6b,0x34)
grad = Image.new("RGBA", (S, S))
gd = ImageDraw.Draw(grad)
for y in range(S):
    t = y/(S-1)
    gd.line([(0,y),(S,y)], fill=tuple(int(top[i]+(bot[i]-top[i])*t) for i in range(3))+(255,))
mask = Image.new("L", (S,S), 0)
ImageDraw.Draw(mask).rounded_rectangle([0,0,S-1,S-1], radius=48*k, fill=255)
img.paste(grad, (0,0), mask)
d = ImageDraw.Draw(img)
# engrenagem branca
cx, cy = 116*k, 140*k
R, r_in, teeth = 78*k, 62*k, 10
pts = []
for i in range(teeth*4):
    a = 2*math.pi*i/(teeth*4) - math.pi/2
    rr = R if (i % 4) in (1,2) else r_in
    pts.append((cx+rr*math.cos(a), cy+rr*math.sin(a)))
d.polygon(pts, fill=(255,255,255,235))
d.ellipse([cx-26*k, cy-26*k, cx+26*k, cy+26*k], fill=(0x2a,0x8a,0x45,255))
# folha verde-clara com contorno branco, sobre a engrenagem
def folha(cx, cy, L, W, ang):
    p = []
    n = 60
    for i in range(n+1):
        t = i/n
        x = -L/2 + L*t; y = W*math.sin(math.pi*t)
        p.append((x, y))
    for i in range(n, -1, -1):
        t = i/n
        x = -L/2 + L*t; y = -W*math.sin(math.pi*t)*0.85
        p.append((x, y))
    ca, sa = math.cos(ang), math.sin(ang)
    return [(cx + x*ca - y*sa, cy + x*sa + y*ca) for x, y in p]
ang = -math.pi/4
fc = (172*k, 104*k)
d.polygon(folha(fc[0], fc[1], 150*k, 42*k, ang), fill=(255,255,255,255))
d.polygon(folha(fc[0], fc[1], 128*k, 32*k, ang), fill=(0x3f,0xb5,0x5a,255))
# nervura
ca, sa = math.cos(ang), math.sin(ang)
a0 = (fc[0]-62*k*ca, fc[1]-62*k*sa); a1 = (fc[0]+56*k*ca, fc[1]+56*k*sa)
d.line([a0, a1], fill=(255,255,255,255), width=int(7*k))
# cabo
d.line([a0, (a0[0]-18*k*ca+6*k, a0[1]-18*k*sa+10*k)], fill=(255,255,255,255), width=int(9*k))
for n in (256, 64):
    img.resize((n,n), Image.LANCZOS).save(f"{out}/ambiental_{n}.png")
img.resize((256,256), Image.LANCZOS).save(f"{out}/ambiental.ico", sizes=[(16,16),(24,24),(32,32),(48,48),(64,64),(128,128),(256,256)])
