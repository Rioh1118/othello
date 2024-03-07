# jsonの設計
## マッチメイキング
リクエスト: 対戦相手を探すために送信。
レスポンス： マッチング成立時のプレイヤー情報とゲームセッション情報
エンドポイント : /matchmake
{
    "request": {
        "name": "display"
        "playerId": "unique_player_id"
    },
    "response": {
        "sessionId": "unique_session_id",
        "players": [
            {
                "playerId": "player1",
                "color": white
            },
            {
                "playerId": "player2",
                "color": "black"
            }
        ],
    }
    "status": "waiting_for_move(in_progress)",
    "board": 
}