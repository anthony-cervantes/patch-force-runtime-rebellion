#!/usr/bin/env python3
"""Generate original short SFX for Patch Force.

The sounds are simple deterministic oscillator/noise envelopes and do not use
third-party samples.
"""

from __future__ import annotations

import math
import random
import struct
import wave
from pathlib import Path

SAMPLE_RATE = 44_100
ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "assets" / "sfx"


def env(t: float, duration: float, attack: float = 0.01, release: float = 0.08) -> float:
    if t < attack:
        return t / attack
    remaining = max(0.0, duration - t)
    if remaining < release:
        return remaining / release
    return 1.0


def write(name: str, duration: float, sample_fn) -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    path = OUT / f"{name}.wav"
    count = int(SAMPLE_RATE * duration)
    with wave.open(str(path), "wb") as wav:
        wav.setnchannels(1)
        wav.setsampwidth(2)
        wav.setframerate(SAMPLE_RATE)
        frames = bytearray()
        for i in range(count):
            t = i / SAMPLE_RATE
            value = max(-1.0, min(1.0, sample_fn(t, duration)))
            frames.extend(struct.pack("<h", int(value * 32767)))
        wav.writeframes(frames)


def sine(freq: float, t: float) -> float:
    return math.sin(t * freq * math.tau)


def square(freq: float, t: float) -> float:
    return 1.0 if sine(freq, t) >= 0.0 else -1.0


def main() -> None:
    rng = random.Random(42)

    write(
        "shoot",
        0.10,
        lambda t, d: 0.42
        * env(t, d, 0.002, 0.06)
        * (0.70 * square(820.0 + 900.0 * t, t) + 0.30 * sine(1640.0, t)),
    )

    write(
        "pickup",
        0.22,
        lambda t, d: 0.40
        * env(t, d, 0.004, 0.09)
        * (sine(660.0 + 880.0 * t, t) + 0.45 * sine(1320.0 + 520.0 * t, t)),
    )

    write(
        "hit",
        0.16,
        lambda t, d: 0.36
        * env(t, d, 0.001, 0.10)
        * (0.50 * square(145.0 - 40.0 * t, t) + 0.50 * rng.uniform(-1.0, 1.0)),
    )

    write(
        "checkpoint",
        0.32,
        lambda t, d: 0.38
        * env(t, d, 0.008, 0.12)
        * (sine(440.0, t) + 0.55 * sine(660.0, t) + 0.35 * sine(880.0, t)),
    )

    write(
        "boss",
        0.42,
        lambda t, d: 0.42
        * env(t, d, 0.02, 0.18)
        * (0.55 * square(92.0 - 20.0 * t, t) + 0.45 * sine(184.0 + 70.0 * t, t)),
    )

    write(
        "victory",
        0.58,
        lambda t, d: 0.36
        * env(t, d, 0.008, 0.18)
        * (
            sine(523.25, t)
            + 0.65 * sine(659.25 if t > 0.14 else 523.25, t)
            + 0.45 * sine(783.99 if t > 0.28 else 659.25, t)
        ),
    )


if __name__ == "__main__":
    main()
