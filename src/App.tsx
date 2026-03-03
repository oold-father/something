import { useEffect, useState } from 'react';
import SearchBar from './components/SearchBar';
import TagPanel from './components/TagPanel';
import WatchedDirectories from './components/WatchedDirectories';
import FileList from './components/FileList';
import { useStore } from './stores/useStore';
import { api } from './lib/api';

export default function App() {
  const [activeTab, setActiveTab] = useState<'tags' | 'directories'>('tags');
  const searchKeywords = useStore((s) => s.searchKeywords);
  const searchOperator = useStore((s) => s.searchOperator);
  const setSearchResults = useStore((s) => s.setSearchResults);
  const setIsLoading = useStore((s) => s.setIsLoading);
  const setStats = useStore((s) => s.setStats);
  const setWatchedDirectories = useStore((s) => s.setWatchedDirectories);

  useEffect(() => {
    loadInitialData();
  }, []);

  useEffect(() => {
    if (searchKeywords.length > 0) {
      performSearch();
    } else {
      setSearchResults(null);
    }
  }, [searchKeywords, searchOperator]);

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
        <div className="p-4 border-b border-border">
          <SearchBar />
        </div>

        {/* 文件列表 */}
        <div className="flex-1">
          <FileList />
        </div>
      </div>
    </div>
  );
}
