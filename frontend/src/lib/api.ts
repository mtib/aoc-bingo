import { config } from './config';

/**
 * API client for making requests to the backend
 */
export const api = {
  /**
   * Make a GET request to the backend
   */
  async get<T>(endpoint: string): Promise<T> {
    const response = await fetch(`${config.backendUrl}${endpoint}`);
    if (!response.ok) {
      throw new Error(`API error: ${response.status} ${response.statusText}`);
    }
    return response.json();
  },

  /**
   * Make a POST request to the backend
   */
  async post<T>(endpoint: string, data?: unknown): Promise<T> {
    const response = await fetch(`${config.backendUrl}${endpoint}`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: data ? JSON.stringify(data) : undefined,
    });
    if (!response.ok) {
      throw new Error(`API error: ${response.status} ${response.statusText}`);
    }
    return response.json();
  },

  /**
   * Make a PUT request to the backend
   */
  async put<T>(endpoint: string, data?: unknown): Promise<T> {
    const response = await fetch(`${config.backendUrl}${endpoint}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
      },
      body: data ? JSON.stringify(data) : undefined,
    });
    if (!response.ok) {
      throw new Error(`API error: ${response.status} ${response.statusText}`);
    }
    return response.json();
  },

  /**
   * Make a DELETE request to the backend
   */
  async delete<T>(endpoint: string): Promise<T> {
    const response = await fetch(`${config.backendUrl}${endpoint}`, {
      method: 'DELETE',
    });
    if (!response.ok) {
      throw new Error(`API error: ${response.status} ${response.statusText}`);
    }
    return response.json();
  },
};
