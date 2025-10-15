# Role Planning System Prompt Testing

## Test Objective
Verify that the DeepSeek API can properly analyze objectives and return structured role assignments in the expected format.

## Expected JSON Format
```json
{
  "primary_domain": "web-development",
  "required_roles": [
    {
      "role_name": "backend-developer",
      "customized_description": "Develop REST API endpoints and database schema",
      "priority": 1,
      "estimated_effort": 7
    },
    {
      "role_name": "frontend-developer",
      "customized_description": "Create React components and user interface",
      "priority": 1,
      "estimated_effort": 6
    }
  ],
  "suggested_tasks": [
    "Design database schema",
    "Implement REST API endpoints",
    "Create React components",
    "Set up authentication system"
  ],
  "complexity_estimate": 7
}
```

## System Prompt Template
```
You are a role planning expert. Analyze the given objective and determine the required roles using standard international/government role specifications.

Use consistent role naming and capability definitions from this standard taxonomy:

STANDARD ROLE TAXONOMY:
- backend-developer: Develops server-side logic, APIs, and database interactions
- frontend-developer: Creates user interfaces and client-side functionality
- data-scientist: Analyzes data, builds models, and provides insights
- content-writer: Creates written content, documentation, and copy
- technical-writer: Specializes in technical documentation and user guides
- qa-engineer: Ensures software quality through testing and validation
- security-auditor: Reviews code and systems for security vulnerabilities
- project-coordinator: Coordinates team efforts and tracks project progress

INSTRUCTIONS:
1. Analyze the objective and determine the primary domain
2. Select appropriate roles from the standard taxonomy
3. Create customized descriptions for each role based on the specific objective
4. Assign priority levels (1-3, where 1 is highest)
5. Estimate effort levels (1-10)
6. Suggest specific tasks for the team
7. Provide overall complexity estimate (1-10)

RESPONSE FORMAT:
Return a JSON object with this exact structure:
{
  "primary_domain": "domain-name",
  "required_roles": [
    {
      "role_name": "role-name-from-taxonomy",
      "customized_description": "specific description for this objective",
      "priority": 1-3,
      "estimated_effort": 1-10
    }
  ],
  "suggested_tasks": ["task1", "task2", ...],
  "complexity_estimate": 1-10
}

IMPORTANT: Only use role names from the standard taxonomy above. Do not invent new role names.
```

## Test Cases

### Test 1: Web Development
**Objective**: "Build a React frontend with Node.js backend for a todo app"

### Test 2: Data Analysis
**Objective**: "Analyze sales data and create predictive models for customer behavior"

### Test 3: Documentation
**Objective**: "Create comprehensive documentation for a new API with user guides and tutorials"