import { useEffect, useState } from 'react';
import SearchBar from './components/SearchBar';
import TagPanel from './components/TagPanel';
import FileList from './components/FileList';
import { useStore } from './stores/useStore';
import { api } from './lib/api';

export default function App() {
  const searchKeywords = useStore((s) => s.searchKeywords);
  const searchOperator = useStore((s) => s.searchOperator);
  const setSearchResults = useStore((s) => s.setSearchResults);
  const setIsLoading = useStore((s) => s.setIsLoading);
  const setStats = useStore((s) => s.setStats);
  const setWatchedDirectories = useStore((s) => s.setWatchedDirectories);

  useEffect(() => {
    // 加载初始数据
    loadInitialData();
  }, []);

  useEffect(() => {
    // 当搜索关键字变化时执行搜索
    if (searchKeywords.length > 0) {
      performSearch();
    } else {
      setSearchResults(null);
    }
  }, [searchKeywords, searchOperator]);

  const loadInitialData = async () => {
    try {
      // 加载系统统计
      const stats = await api.getStats();
      setStats(stats);

      // 加载监控目录
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
      {/* 左侧标签面板 */}
      <div className="w-64 flex-shrink-0">
        <TagPanel />
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
