/* Gravitational-lensing hero field.
   A regular lattice bent by a point mass (thin-lens equation), an Einstein ring
   lit where the deflection peaks, over a faint starfield. Light bending around
   mass — the "Lensing" idea made literal. */
(function () {
    "use strict";
    var canvas = document.getElementById("lens-canvas");
    if (!canvas || !canvas.getContext) return;
    var ctx = canvas.getContext("2d");
    var reduce = window.matchMedia("(prefers-reduced-motion: reduce)").matches;

    var W = 0, H = 0, dpr = 1;
    var cx = 0, cy = 0;          // mass centre
    var px = 0, py = 0;          // parallax target (0..1 around centre)
    var Re = 0;                  // Einstein radius (animated on load)
    var ReTarget = 0;
    var stars = [];
    var start = null;
    var raf = 0;

    function resize() {
        dpr = Math.min(window.devicePixelRatio || 1, 2);
        W = canvas.clientWidth;
        H = canvas.clientHeight;
        canvas.width = Math.round(W * dpr);
        canvas.height = Math.round(H * dpr);
        ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
        ReTarget = Math.min(W, H) * 0.17;
        seedStars();
    }

    function seedStars() {
        stars = [];
        var n = Math.round((W * H) / 14000);
        n = Math.max(40, Math.min(180, n));
        for (var i = 0; i < n; i++) {
            stars.push({
                x: Math.random() * W,
                y: Math.random() * H,
                r: Math.random() * 1.1 + 0.2,
                p: Math.random() * Math.PI * 2,
                s: Math.random() * 0.6 + 0.2
            });
        }
    }

    // Thin-lens point-mass map: source radius r -> image radius r'.
    function lensScale(r) {
        if (r < 0.0001) return 1;
        var rp = 0.5 * (r + Math.sqrt(r * r + 4 * Re * Re));
        return rp / r;
    }

    function draw(t) {
        var elapsed = start == null ? 0 : (t - start) / 1000;
        // ease Re in over 1.5s
        var k = reduce ? 1 : Math.min(1, elapsed / 1.5);
        var ease = 1 - Math.pow(1 - k, 3);
        Re = ReTarget * ease;
        var breathe = reduce ? 0 : Math.sin(elapsed * 0.7) * 0.02;
        Re *= 1 + breathe;

        // parallax + idle drift of the mass
        var baseX = W * 0.66, baseY = H * 0.44;
        var driftX = reduce ? 0 : Math.cos(elapsed * 0.18) * W * 0.015;
        var driftY = reduce ? 0 : Math.sin(elapsed * 0.23) * H * 0.02;
        var tx = baseX + (px - 0.5) * W * 0.06 + driftX;
        var ty = baseY + (py - 0.5) * H * 0.06 + driftY;
        cx += (tx - cx) * (reduce ? 1 : 0.05);
        cy += (ty - cy) * (reduce ? 1 : 0.05);

        var rot = reduce ? 0 : elapsed * 0.012;
        var appear = reduce ? 1 : Math.min(1, elapsed / 1.1);

        ctx.clearRect(0, 0, W, H);
        ctx.globalAlpha = appear;

        drawStars(elapsed);
        drawLattice(rot);
        drawRing();
        drawMass();

        ctx.globalAlpha = 1;

        if (!reduce) raf = window.requestAnimationFrame(draw);
    }

    function drawStars(elapsed) {
        for (var i = 0; i < stars.length; i++) {
            var s = stars[i];
            var tw = reduce ? 0.7 : 0.5 + 0.5 * Math.sin(elapsed * s.s + s.p);
            ctx.beginPath();
            ctx.arc(s.x, s.y, s.r, 0, Math.PI * 2);
            ctx.fillStyle = "rgba(200,208,240," + (0.10 + tw * 0.35) + ")";
            ctx.fill();
        }
    }

    function project(x, y, rot) {
        // rotate around centre, then apply lensing displacement
        var dx = x - cx, dy = y - cy;
        if (rot) {
            var cosr = Math.cos(rot), sinr = Math.sin(rot);
            var rx = dx * cosr - dy * sinr;
            var ry = dx * sinr + dy * cosr;
            dx = rx; dy = ry;
        }
        var r = Math.sqrt(dx * dx + dy * dy);
        var sc = lensScale(r);
        return [cx + dx * sc, cy + dy * sc];
    }

    function drawLattice(rot) {
        var pad = Math.max(W, H) * 0.6;
        var minX = cx - pad, maxX = cx + pad;
        var minY = cy - pad, maxY = cy + pad;
        var step = Math.max(38, Math.min(W, H) / 13);
        var sample = Math.max(6, step / 4);

        // Draw each segment with an alpha that rises near the Einstein ring —
        // gravitational magnification: the lattice brightens where light piles up.
        ctx.lineWidth = 1;
        var x, y, p, prev, r, mag, a;

        function seg(x0, y0, x1, y1) {
            p = project(x1, y1, rot);
            if (prev) {
                r = Math.sqrt((x1 - cx) * (x1 - cx) + (y1 - cy) * (y1 - cy));
                mag = Re > 0 ? Math.exp(-Math.abs(r - Re * 1.3) / (Re * 1.1)) : 0;
                a = 0.09 + mag * 0.5;
                ctx.strokeStyle = "rgba(176,190,255," + a.toFixed(3) + ")";
                ctx.beginPath();
                ctx.moveTo(prev[0], prev[1]);
                ctx.lineTo(p[0], p[1]);
                ctx.stroke();
            }
            prev = p;
        }

        for (x = Math.floor(minX / step) * step; x <= maxX; x += step) {
            prev = null;
            for (y = minY; y <= maxY; y += sample) seg(x, y - sample, x, y);
        }
        for (y = Math.floor(minY / step) * step; y <= maxY; y += step) {
            prev = null;
            for (x = minX; x <= maxX; x += sample) seg(x - sample, y, x, y);
        }
    }

    function drawRing() {
        if (Re < 1) return;
        // outer glow
        var glow = ctx.createRadialGradient(cx, cy, Re * 0.55, cx, cy, Re * 1.7);
        glow.addColorStop(0, "rgba(255,205,130,0)");
        glow.addColorStop(0.72, "rgba(255,196,120,0.16)");
        glow.addColorStop(0.85, "rgba(255,214,150,0.30)");
        glow.addColorStop(1, "rgba(255,205,130,0)");
        ctx.fillStyle = glow;
        ctx.beginPath();
        ctx.arc(cx, cy, Re * 1.7, 0, Math.PI * 2);
        ctx.fill();

        // bright ring stroke
        ctx.save();
        ctx.shadowColor = "rgba(255,210,140,0.9)";
        ctx.shadowBlur = 22;
        ctx.beginPath();
        ctx.arc(cx, cy, Re, 0, Math.PI * 2);
        ctx.strokeStyle = "rgba(255,232,190,0.85)";
        ctx.lineWidth = 1.6;
        ctx.stroke();
        ctx.restore();
    }

    function drawMass() {
        var g = ctx.createRadialGradient(cx, cy, 0, cx, cy, Re * 0.9);
        g.addColorStop(0, "rgba(8,8,18,0.95)");
        g.addColorStop(0.7, "rgba(12,12,26,0.7)");
        g.addColorStop(1, "rgba(12,12,26,0)");
        ctx.fillStyle = g;
        ctx.beginPath();
        ctx.arc(cx, cy, Re * 0.9, 0, Math.PI * 2);
        ctx.fill();
    }

    function onPointer(e) {
        px = e.clientX / window.innerWidth;
        py = e.clientY / window.innerHeight;
    }

    function init() {
        resize();
        cx = W * 0.66; cy = H * 0.44;
        window.addEventListener("resize", resize, { passive: true });
        if (!reduce && window.matchMedia("(pointer:fine)").matches) {
            window.addEventListener("pointermove", onPointer, { passive: true });
        }
        raf = window.requestAnimationFrame(function step(t) {
            if (start == null) start = t;
            draw(t);
        });
    }

    if (document.readyState === "loading") {
        document.addEventListener("DOMContentLoaded", init);
    } else {
        init();
    }
})();
