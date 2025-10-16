- DeepSeek API call example:
```
curl https://api.deepseek.com/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer ${DEEPSEEK_API_KEY}" \
  -d '{
        "model": "deepseek-chat",
        "messages": [
          {"role": "system", "content": "You are a helpful assistant."},
          {"role": "user", "content": "Hello!"}
        ],
        "stream": false
      }'
```

- DeepSeek API reference: https://api-docs.deepseek.com/api/create-chat-completion


- No shared test key is bundled with this repository. Create an API key in your DeepSeek account and export it locally, for example:
  ```
  export DEEPSEEK_API_KEY="sk-your-own-key"
  ```
