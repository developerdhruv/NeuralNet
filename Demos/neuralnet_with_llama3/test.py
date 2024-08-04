from fastapi import FastAPI, Query
from typing import Optional
import neuralnetrag as rag
import requests
import json
from fastapi.middleware.cors import CORSMiddleware

app = FastAPI()
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

def ragint(url, query, k=1, chunk_size=1000):
    relevant_chunks = rag.retrieve(url, query, k, chunk_size)
    return relevant_chunks

def ollam_llm(user_input, relevant_docs):
    prompt = """You are a bot that makes recommendations for Tech and provides answers. Answer in a brief way and include deep concepts, and answer in a paragraph format. The recommended data is: {relevant_docs}. The user input is: {user_input}. Compile the answer based on the given relevant data and the user input."""
    url = 'http://localhost:11434/api/generate'
    data = {
        "model": "llama3",
        "prompt": prompt.format(user_input=user_input, relevant_docs=relevant_docs),
    }
    headers = {'Content-Type': 'application/json'}
    response = requests.post(url, data=json.dumps(data), headers=headers, stream=True)
    full_response = []
    try:
        for line in response.iter_lines():
            if line:
                decoded_line = json.loads(line.decode('utf-8'))
                full_response.append(decoded_line['response'])
    finally:
        response.close()
    return ''.join(full_response)



@app.get("/chat")
async def chat(user_input: str = Query(...), url: str = Query(...)):
    relevent_docs = ragint(url, user_input)
    response = ollam_llm(user_input, relevent_docs)
    return {"response": response}
