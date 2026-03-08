import { useEffect, useState } from 'react';
import { Settings, Sun, Moon } from 'lucide-react';
import SearchBar from './components/SearchBar';
import TagPanel from './components/TagPanel';
import WatchedDirectories from './components/WatchedDirectories';
import FileList from './components/FileList';
import Toast from './components/Toast';
import SettingsPanel from './components/SettingsPanel';
import { useStore } from './stores/useStore';
import { api } from './lib/api';

export default function App() {
  const [activeTab, setActiveTab] = useState<'tags' | 'directories'>('tags');
  const [showSettings, setShowSettings] = useState(false);

  console.log('[App] Rendered, showSettings:', showSettings);

  const searchKeywords = useStore((s) => s.searchKeywords);
  const searchOperator = useStore((s) => s.searchOperator);
  const selectedTags = useStore((s) => s.selectedTags);
  const setSearchResults = useStore((s) => s.setSearchResults);
  const setIsLoading = useStore((s) => s.setIsLoading);
  const setStats = useStore((s) => s.setStats);
  const setWatchedDirectories = useStore((s) => s.setWatchedDirectories);
  const updateSettings = useStore((s) => s.updateSettings);
  const theme = useStore((s) => s.theme);
  const setTheme = useStore((s) => s.setTheme);

  useEffect(() => {
    loadInitialData();
  }, []);

  // 禁用默认右键菜单（除文件列表外）
  useEffect(() => {
    const handleContextMenu = (e: MouseEvent) => {
      // 检查点击目标是否在文件列表区域内
      const target = e.target as HTMLElement;
      const fileListElement = target.closest('[role="listitem"]');

      // 只有在文件列表项上才允许右键
      if (fileListElement) {
        // 允许右键，不阻止默认行为
        return;
      }

      // 其他位置阻止右键菜单
      e.preventDefault();
      e.stopPropagation();
    };

    document.addEventListener('contextmenu', handleContextMenu as EventListener, false);
    return () => {
      document.removeEventListener('contextmenu', handleContextMenu as EventListener);
    };
  }, []);

  useEffect(() => {
    if (searchKeywords.length > 0) {
      performSearch();
    } else {
      setSearchResults(null);
    }
  }, [searchKeywords, searchOperator, selectedTags]);

  const loadInitialData = async () => {
    try {
      const stats = await api.getStats();
      setStats(stats);

      const dirs = await api.getWatchedDirectories();
      setWatchedDirectories(dirs);
    } catch (error) {
      console.error('加载初始数据失败:', error);
    }
  };

  const performSearch = async () => {
    setIsLoading(true);
    try {
      const results = await api.searchFiles(
        searchKeywords,
        searchOperator,
        undefined,
        selectedTags.length > 0 ? selectedTags : undefined,
        50,
        0
      );
      setSearchResults(results);
    } catch (error) {
      console.error('搜索失败:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const toggleTheme = () => {
    setTheme(theme === 'dark' ? 'light' : 'dark');
  };

  return (
    <div className="flex h-screen bg-background text-foreground">
      {/* 左侧面板 */}
      <div className="w-72 flex-shrink-0 flex flex-col bg-card border-r border-border">
        {/* 标签切换 */}
        <div className="flex border-b border-border">
          <button
            onClick={() => setActiveTab('tags')}
            className={`flex-1 px-4 py-3 text-sm font-medium transition-colors ${
              activeTab === 'tags'
                ? 'bg-primary text-primary-foreground border-b-2 border-primary'
                : 'text-muted-foreground hover:text-foreground'
            }`}
          >
            标签
          </button>
          <button
            onClick={() => setActiveTab('directories')}
            className={`flex-1 px-4 py-3 text-sm font-medium transition-colors ${
              activeTab === 'directories'
                ? 'bg-primary text-primary-foreground border-b-2 border-primary'
                : 'text-muted-foreground hover:text-foreground'
            }`}
          >
            监控目录
          </button>
        </div>
        {/* 内容区域 */}
        <div className="flex-1 overflow-y-auto">
          {activeTab === 'tags' ? <TagPanel /> : <WatchedDirectories />}
        </div>
      </div>

      {/* 右侧主内容区 */}
      <div className="flex-1 flex flex-col min-w-0">
        {/* 顶部搜索栏 */}
        <div className="p-4 border-b border-border flex items-center gap-4">
          <div className="flex-1">
            <SearchBar />
          </div>
          <div className="flex gap-2">
            <button
              onClick={toggleTheme}
              className="p-2 text-gray-600 hover:text-blue-600 hover:bg-blue-50 dark:hover:bg-blue-900/20 rounded-lg transition-colors"
              title={theme === 'dark' ? '切换到浅色模式' : '切换到深色模式'}
            >
              {theme === 'dark' ? <Sun size={20} /> : <Moon size={20} />}
            </button>
            <button
              onClick={() => {
                console.log('[App] Settings button clicked');
                setShowSettings(true);
              }}
              className="p-2 text-gray-600 hover:text-blue-600 hover:bg-blue-50 dark:hover:bg-blue-900/20 rounded-lg transition-colors"
              title="设置"
            >
              <Settings size={20} />
            </button>
          </div>
        </div>

        {/* 文件列表 */}
        <div className="flex-1">
          <FileList />
        </div>
      </div>

      {/* Toast 通知 */}
      <Toast />

      {/* 设置面板 */}
      <SettingsPanel
        isOpen={showSettings}
        onClose={() => setShowSettings(false)}
        onSave={updateSettings}
      />
    </div>
  );
}
