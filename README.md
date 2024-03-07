# jsonの設計
## マッチメイキング
リクエスト: 対戦相手を探すために送信。
レスポンス： マッチング成立時のプレイヤー情報とゲームセッション情報
エンドポイント : /matchmake
{
    "request": {
        "gameId": "unique_game_id"
        "player":{
            "name": "display",
            "playerId": "unique_player_id",
        },
        "status": "waiting_for_opponent" // or "game_started"
    },
    "response": {
        "sessionId": "unique_session",
        "player_color": "black" // or "white"
        "opponent": {
            "name": "display",
            "playerId" : "unique"
        },
        "status": "waiting_for_opponent" // or "game started",
        "board": "initial_board_infomation"
    }
}

## 盤面の更新
// websocketで両方のプレイヤーに通知 ValidMovesのレンダリングはフロントエンド
{
  "sessionId": "unique_session_id",
  "board": [
    // 盤面の現在の状態を2次元配列などで表現
  ],
  "nextPlayer": "unique_player_id",
  "validMoves": [
    // 有効な手のリスト
    {"x": 4, "y": 2},
    {"x": 5, "y": 3}
    // その他有効な手
  ],
  "status": "game_in_progress" // or "game_over"
}
## 手の送信

{
    "sessionId": "unique_session_id",
    "playerId": "unique_player_id",
    "move": "unique_player_id",
    "move": {
        "x": 3,
        "y": 2
    }
}