/**
 * 搜索栏组件
 */
import { useState } from 'react';
import { Search, X } from 'lucide-react';
import { useStore } from '../../stores/useStore';

export default function SearchBar() {
  const [inputValue, setInputValue] = useState('');
  const [searchOperator, setSearchOperator] = useState<'AND' | 'OR'>('AND');
  const setSearchKeywords = useStore((s) => s.setSearchKeywords);
  const storeSearchOperator = useStore((s) => s.searchOperator);
  const storeSetSearchOperator = useStore((s) => s.setSearchOperator);
  const setIsLoading = useStore((s) => s.setIsLoading);

  const handleSearch = async () => {
    const keywords = inputValue.trim().split(/\s+/).filter(Boolean);
    if (keywords.length === 0) return;

    setSearchKeywords(keywords);
    storeSetSearchOperator(searchOperator);
    setIsLoading(true);

    // 触发搜索逻辑（在主应用中处理）
    setInputValue('');
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      handleSearch();
    }
  };

  const clearSearch = () => {
    setInputValue('');
    setSearchKeywords([]);
  };

  return (
    <div className="flex items-center gap-2 bg-card border border-border rounded-lg p-2">
      <Search className="w-5 h-5 text-muted-foreground" />
      <input
        type="text"
        placeholder="搜索文件..."
        value={inputValue}
        onChange={(e) => setInputValue(e.target.value)}
        onKeyDown={handleKeyDown}
        className="flex-1 bg-transparent outline-none text-foreground placeholder:text-muted-foreground"
      />
      {storeSearchOperator && (
        <button
          onClick={() => storeSetSearchOperator(searchOperator === 'AND' ? 'OR' : 'AND')}
          className="px-3 py-1 text-sm rounded bg-secondary text-secondary-foreground hover:bg-secondary/80 transition-colors"
        >
          {searchOperator}
        </button>
      )}
      {inputValue && (
        <button onClick={clearSearch} className="p-1 hover:bg-muted rounded transition-colors">
          <X className="w-4 h-4 text-muted-foreground" />
        </button>
      )}
    </div>
  );
}
