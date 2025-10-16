test command:
```
env DEEPSEEK_API_KEY=your-deepseek-api-key target/debug/codex multi-agent --config model_provider=deepseek --config model=deepseek-reasone
```

when enter interactive mode, inputed objective "write a python script to add two numbers and output results, should be verified" and comfirmed the plan and tasks, it reports 
"🔮 Task is running in background mode
Use 'codex multi-agent --monitor --task-id 8b9780ba-b7b7-495d-b2d4-b9d8ae795f85' to continuously monitor progress
Use 'codex multi-agent --interactive --task-id 8b9780ba-b7b7-495d-b2d4-b9d8ae795f85' to engage with the task
" and exits.
the script file is not exist, early quit, and why background mode?
