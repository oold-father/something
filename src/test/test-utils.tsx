import { render, RenderOptions } from '@testing-library/react';
import { ReactElement } from 'react';

// 创建一个简单的渲染函数，不使用复杂的 mock
export function renderWithProviders(ui: ReactElement, options?: RenderOptions) {
  return render(ui, {
    ...options,
    // 可以添加 wrapper provider
  });
}
