import { describe, it, expect } from 'vitest';
import { ApiError } from '@/lib/api/types';

describe('ApiError', () => {
  it('creates error with statusCode and message', () => {
    const error = new ApiError(404, 'Not found');
    expect(error.statusCode).toBe(404);
    expect(error.message).toBe('Not found');
    expect(error.name).toBe('ApiError');
  });

  it('is an instance of Error', () => {
    const error = new ApiError(500, 'Server error');
    expect(error).toBeInstanceOf(Error);
    expect(error).toBeInstanceOf(ApiError);
  });

  it('preserves status code for different HTTP errors', () => {
    const errors = [
      new ApiError(400, 'Bad Request'),
      new ApiError(401, 'Unauthorized'),
      new ApiError(403, 'Forbidden'),
      new ApiError(409, 'Conflict'),
      new ApiError(500, 'Internal Server Error'),
    ];

    expect(errors.map((e) => e.statusCode)).toEqual([400, 401, 403, 409, 500]);
  });
});
