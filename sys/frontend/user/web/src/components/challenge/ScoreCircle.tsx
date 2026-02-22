'use client';

import { motion } from 'framer-motion';

interface ScoreCircleProps {
  score: number;
}

const RADIUS = 54;
const CIRCUMFERENCE = 2 * Math.PI * RADIUS;

export function ScoreCircle({ score }: ScoreCircleProps) {
  const offset = CIRCUMFERENCE - (score / 100) * CIRCUMFERENCE;

  return (
    <div className="flex flex-col items-center">
      <svg width={140} height={140} viewBox="0 0 140 140">
        <defs>
          <linearGradient id="scoreGrad" x1="0%" y1="0%" x2="100%" y2="100%">
            <stop offset="0%" stopColor="#667eea" />
            <stop offset="100%" stopColor="#764ba2" />
          </linearGradient>
        </defs>
        <circle
          cx={70}
          cy={70}
          r={RADIUS}
          fill="none"
          stroke="#E0E0E0"
          strokeWidth={10}
        />
        <motion.circle
          cx={70}
          cy={70}
          r={RADIUS}
          fill="none"
          stroke="url(#scoreGrad)"
          strokeWidth={10}
          strokeLinecap="round"
          strokeDasharray={CIRCUMFERENCE}
          initial={{ strokeDashoffset: CIRCUMFERENCE }}
          animate={{ strokeDashoffset: offset }}
          transition={{ duration: 1.5, ease: 'easeOut' }}
          transform="rotate(-90 70 70)"
        />
        <text
          x={70}
          y={65}
          textAnchor="middle"
          className="fill-[#1a1a2e] text-3xl font-bold"
          dominantBaseline="central"
        >
          {score}
        </text>
        <text
          x={70}
          y={90}
          textAnchor="middle"
          className="fill-[#999999] text-xs"
        >
          点
        </text>
      </svg>
    </div>
  );
}
