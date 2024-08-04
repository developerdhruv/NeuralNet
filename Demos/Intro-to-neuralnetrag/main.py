
import neuralnetrag as rag
url = "https://medium.com/llamaindex-blog/building-better-tools-for-llm-agents-f8c5a6714f11"
query = "AI and ai agents components"
k =1
chunk_size = 1000 
relevant_chunks = rag.retrieve(url, query, k, chunk_size)
print(relevant_chunks)

