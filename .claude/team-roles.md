# 开发团队角色分工

## 团队信息
- **团队名称**: development-team
- **团队描述**: 软件开发团队 - 包含架构师、前端开发者、后端开发者、DevOps和测试工程师

## 角色定义

### 1. Product Manager（产品经理）
- **职责**:
  - 与用户沟通，深入了解需求细节
  - 进行需求分析和需求梳理
  - 确保需求按照用户的想法实现
  - 将用户需求转化为清晰的任务描述
  - 验证实现结果是否符合预期
- **成员**: product-manager@development-team

### 2. Planner（架构师）
- **职责**:
  - 拆解复杂任务，制定重构方案
  - 设计系统架构和技术方案
  - 确保技术决策的合理性和一致性
- **成员**: planner@development-team

### 2. Frontend Coder（前端开发者）
- **职责**:
  - 实现前端页面和组件
  - 处理前端相关的bug修改
  - 确保前端代码质量和用户体验
- **成员**: frontend-coder@development-team

### 3. Backend Coder（后端开发者）
- **职责**:
  - 实现后端API和数据库操作
  - 处理后端相关的bug修改
  - 确保后端代码质量和性能
- **成员**: backend-coder@development-team

### 4. DevOps（开发运维）
- **职责**:
  - bug的定界和定位（判断是前端问题还是后端问题）
  - 环境配置和部署相关问题
  - 协助排查系统级别的问题
- **成员**: devops@development-team

### 5. Tester（测试工程师）
- **职责**:
  - 功能测试和验证
  - 编写测试用例
  - 确保功能按预期工作
  - 审计coder修改bug提交的代码，确保不会影响其他正常可以使用的功能
- **成员**: tester@development-team

## 工作流程

1. **需求沟通**: 当用户提出新需求时，首先由 Product Manager 与用户沟通，深入了解需求细节
2. **需求分析**: Product Manager 分析需求，确保理解正确，并记录需求细节
3. **任务接收**: 需求明确后，由 Planner 进行任务拆解和方案制定
2. **方案确认**: 方案确认后，根据任务类型分配给对应的 Coder（前端或后端）
3. **问题定位**: 如果出现bug，首先由 DevOps 进行定界（前端/后端），再分配给对应开发者
4. **开发完成**: 开发完成后，由 Tester 进行功能测试和验证
5. **代码审计**: 对于bug修复，Tester需要审计提交的代码是否会影响其他正常功能
6. **任务闭环**: 测试通过且代码审计无误后，任务完成

## 注意事项
- 所有团队成员名称: product-manager, planner, frontend-coder, backend-coder, devops, tester
- 使用 SendMessage 工具与团队成员进行通信
- 创建任务后通过 TaskUpdate 将任务分配给对应的团队成员
