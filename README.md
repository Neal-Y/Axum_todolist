# TODO-list with Axum

## 這個項目是一個用Rust和Axum框架創建的ToDo List Web應用程式。

### 項目介紹
    本項目是一個簡單的任務管理工具，使用者可以在此創建，檢視，修改以及刪除任務。  
    主要是想透過新興的Rust後端框架一步一步練習基本的CRUD。  
    同時使用integration-test保證程式碼的安全。

### 技術棧
    Rust: 使用Rust語言開發，因其出色的效能與安全性。
    Axum: 使用Axum作為後端框架，它是一個新興且高效的Rust Web框架。
    PostgreSQL: 使用PostgreSQL作為數據庫，以儲存任務資訊。
    Docker: 使用Docker來封裝應用和其依賴，簡化部署流程。
    Jest: 使用Jest來進行整合測試，確保應用功能的穩定性。
    
### 功能(CRUD)
    任務創建: 用戶可以創建新的任務。
    任務查詢: 用戶可以查看所有任務的列表。
    任務修改: 用戶可以修改任務的內容和狀態。
    任務刪除: 用戶可以刪除任務。
