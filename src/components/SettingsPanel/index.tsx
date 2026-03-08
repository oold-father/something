/**
 * 设置面板组件
 */
import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useStore } from '../../stores/useStore';
import {
  Settings,
  Palette,
  Search as SearchIcon,
  Folder,
  Save,
  RotateCcw,
  X,
  Wrench,
} from 'lucide-react';

// 添加详细日志
console.log('[SettingsPanel] Component loaded');

export interface AppSettings {
  // 外观设置
  theme: 'light' | 'dark' | 'auto';
  accentColor: string;

  // 搜索设置
  defaultSearchOperator: 'AND' | 'OR';
  searchResultLimit: number;

  // 监控设置
  autoScanInterval: number; // 分钟
  showScanNotifications: boolean;

  // 其他
  showHiddenFiles: boolean;
}

const defaultSettings: AppSettings = {
  theme: 'auto',
  accentColor: '#3b82f6',
  defaultSearchOperator: 'AND',
  searchResultLimit: 50,
  autoScanInterval: 30,
  showScanNotifications: true,
  showHiddenFiles: false,
};

const STORAGE_KEY = 'something_settings';

export default function SettingsPanel({
  isOpen,
  onClose,
  onSave,
}: {
  isOpen: boolean;
  onClose: () => void;
  onSave?: (settings: Partial<AppSettings>) => void;
}) {
  console.log('[SettingsPanel] Render called, isOpen:', isOpen);
  const [settings, setSettings] = useState<AppSettings>(defaultSettings);
  const [hasChanges, setHasChanges] = useState(false);
  const [isFixing, setIsFixing] = useState(false);

  // 加载设置
  useEffect(() => {
    loadSettings();
  }, []);

  const loadSettings = () => {
    try {
      const stored = localStorage.getItem(STORAGE_KEY);
      if (stored) {
        const parsed = JSON.parse(stored);
        console.log('Loaded settings:', parsed);
        setSettings({ ...defaultSettings, ...parsed });
      }
    } catch (error) {
      console.error('加载设置失败:', error);
      // 如果加载失败，使用默认设置
      setSettings(defaultSettings);
    }
  };

  const handleSave = () => {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
    setHasChanges(false);
    onSave?.(settings);
    onClose();
  };

  const handleReset = () => {
    setSettings(defaultSettings);
    setHasChanges(true);
  };

  // 修复标签计数
  const handleFixTagCounts = async () => {
    setIsFixing(true);
    try {
      await invoke('fix_tag_counts');
      console.log('[SettingsPanel] 标签计数修复成功');
      // 刷新标签列表
      const tags = await invoke('get_all_tags');
      useStore.getState().setTags(tags as any[]);
      alert('标签计数已修复，请刷新页面查看');
    } catch (error) {
      console.error('[SettingsPanel] 修复标签计数失败:', error);
      alert('修复失败: ' + error);
    } finally {
      setIsFixing(false);
    }
  };

  const handleInputChange = <K extends keyof AppSettings>(
    key: K,
    value: AppSettings[K]
  ) => {
    setSettings((prev) => ({ ...prev, [key]: value }));
    setHasChanges(true);
  };

  if (!isOpen) {
    console.log('[SettingsPanel] Not rendering, isOpen is false');
    return null;
  }

  console.log('[SettingsPanel] Rendering dialog, settings:', settings);

  const handleBackdropClick = (e: React.MouseEvent) => {
    console.log('[SettingsPanel] Backdrop clicked', { target: e.target, currentTarget: e.currentTarget });
    if (e.target === e.currentTarget) {
      console.log('[SettingsPanel] Closing dialog via backdrop click');
      onClose();
    }
  };

  return (
    <div
      className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4"
      onClick={handleBackdropClick}
    >
      <div
        className="bg-white dark:bg-gray-800 rounded-lg shadow-xl w-full max-w-2xl max-h-[90vh] overflow-hidden flex flex-col"
        onClick={(e) => e.stopPropagation()}
      >
        {/* 头部 */}
        <div className="flex items-center justify-between p-6 border-b border-gray-200 dark:border-gray-700">
          <div className="flex items-center gap-3">
            <Settings className="w-6 h-6 text-blue-600" />
            <h2 className="text-xl font-semibold">设置</h2>
          </div>
          <button
            onClick={onClose}
            className="p-2 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* 内容区域 */}
        <div className="flex-1 overflow-y-auto p-6 space-y-8">
          {/* 外观设置 */}
          <Section
            icon={Palette}
            title="外观"
            description="自定义应用的外观和主题"
          >
            <SettingRow label="主题">
              <select
                value={settings.theme}
                onChange={(e) =>
                  handleInputChange('theme', e.target.value as AppSettings['theme'])
                }
                className="px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100"
              >
                <option value="light">浅色</option>
                <option value="dark">深色</option>
                <option value="auto">跟随系统</option>
              </select>
            </SettingRow>

            <SettingRow label="主题色">
              <div className="flex gap-2">
                {['#3b82f6', '#ef4444', '#10b981', '#f59e0b', '#8b5cf6'].map(
                  (color) => (
                    <button
                      key={color}
                      onClick={() => handleInputChange('accentColor', color)}
                      className={`w-8 h-8 rounded-full border-2 ${
                        settings.accentColor === color
                          ? 'border-gray-900 dark:border-white'
                          : 'border-transparent'
                      }`}
                      style={{ backgroundColor: color }}
                    />
                  )
                )}
              </div>
            </SettingRow>
          </Section>

          {/* 搜索设置 */}
          <Section
            icon={SearchIcon}
            title="搜索"
            description="配置搜索行为和结果显示"
          >
            <SettingRow label="默认搜索逻辑">
              <select
                value={settings.defaultSearchOperator}
                onChange={(e) =>
                  handleInputChange(
                    'defaultSearchOperator',
                    e.target.value as AppSettings['defaultSearchOperator']
                  )
                }
                className="px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100"
              >
                <option value="AND">AND（同时包含所有关键词）</option>
                <option value="OR">OR（包含任一关键词）</option>
              </select>
            </SettingRow>

            <SettingRow label="搜索结果数量">
              <input
                type="number"
                min="10"
                max="200"
                value={settings.searchResultLimit}
                onChange={(e) =>
                  handleInputChange('searchResultLimit', parseInt(e.target.value) || 50)
                }
                className="w-24 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100"
              />
            </SettingRow>
          </Section>

          {/* 监控设置 */}
          <Section
            icon={Folder}
            title="文件监控"
            description="配置文件监控和扫描行为"
          >
            <SettingRow label="自动扫描间隔（分钟）">
              <input
                type="number"
                min="5"
                max="1440"
                value={settings.autoScanInterval}
                onChange={(e) =>
                  handleInputChange('autoScanInterval', parseInt(e.target.value) || 30)
                }
                className="w-24 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100"
              />
            </SettingRow>

            <SettingRow label="显示扫描通知">
              <label className="relative inline-flex items-center cursor-pointer">
                <input
                  type="checkbox"
                  checked={settings.showScanNotifications}
                  onChange={(e) => handleInputChange('showScanNotifications', e.target.checked)}
                  className="sr-only peer"
                />
                <div className="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600" />
              </label>
            </SettingRow>

            <SettingRow label="显示隐藏文件">
              <label className="relative inline-flex items-center cursor-pointer">
                <input
                  type="checkbox"
                  checked={settings.showHiddenFiles}
                  onChange={(e) => handleInputChange('showHiddenFiles', e.target.checked)}
                  className="sr-only peer"
                />
                <div className="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600" />
              </label>
            </SettingRow>
          </Section>
        </div>

        {/* 底部按钮 */}
        <div className="flex items-center justify-between p-6 border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-900/50">
          <button
            onClick={handleReset}
            className="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-700 rounded-lg flex items-center gap-2 transition-colors"
          >
            <RotateCcw size={18} />
            恢复默认
          </button>
          <div className="flex gap-3">
            <button
              onClick={handleFixTagCounts}
              disabled={isFixing}
              className="px-4 py-2 bg-orange-600 text-white rounded-lg hover:bg-orange-700 disabled:opacity-50 flex items-center gap-2 transition-colors"
            >
              <Wrench size={18} />
              {isFixing ? '修复中...' : '修复标签计数'}
            </button>
            <button
              onClick={onClose}
              className="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-700 rounded-lg transition-colors"
            >
              取消
            </button>
            <button
              onClick={handleSave}
              disabled={!hasChanges}
              className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2 transition-colors"
            >
              <Save size={18} />
              保存更改
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}

interface SectionProps {
  icon: any;
  title: string;
  description: string;
  children: React.ReactNode;
}

function Section({ icon: Icon, title, description, children }: SectionProps) {
  return (
    <section>
      <div className="flex items-start gap-3 mb-4">
        <Icon className="w-5 h-5 text-gray-600 dark:text-gray-400 mt-0.5" />
        <div>
          <h3 className="font-semibold text-gray-900 dark:text-gray-100">{title}</h3>
          <p className="text-sm text-gray-500 dark:text-gray-400">{description}</p>
        </div>
      </div>
      <div className="pl-8 space-y-4">{children}</div>
    </section>
  );
}

interface SettingRowProps {
  label: string;
  children: React.ReactNode;
}

function SettingRow({ label, children }: SettingRowProps) {
  return (
    <div className="flex items-center justify-between">
      <span className="text-sm text-gray-700 dark:text-gray-300">{label}</span>
      {children}
    </div>
  );
}
