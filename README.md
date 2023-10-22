# MeiliSearch tools

Very basic set of tools to work with MeiliSearch for personal use.
For now, it only contains very basic tools like:
- Import from a directory with json files into a Melisearch index
- Fetch index stats
- Fetch task stats

## Installation

// TODO

## Usage

* Import from a directory with json files into a Melisearch index

```bash
#Load the documents
mtool import \
		--folder <path to folder> \ 
    --host-api http://localhost:7700  \
    --index-name <index_name> \ 
    --api-key <api_key>
```

* Fetch index stats

```bash
mtool index-stats --host-api http://localhost:7700 --api-key  <api_key>

```

* Fetch task stats

```bash
mtool task-stats --host-api http://localhost:7700 --api-key  <api_key>

```