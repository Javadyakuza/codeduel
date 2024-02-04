# CODE DUEL

![CODEDUEL LOGO](https://github.com/Javadyakuza/codeduel/blob/server/backend/public/codu_l.png)



[DEPLOYED UI](https://code-duel-frontend.vercel.app/%5C)
## CodeDuel: Elevate Your Programming Challenges


Code Duel is a skill based platform where developers and tech corporations can either find the solution for the issues and questions or vice versa, design an solution for other developers and corporations questions and challenges.


## Project Description:

CodeDuel is a dynamic platform designed to revolutionize the programming challenge experience by fostering a big community of "rivals" (question designers) and "daredevils" (problem solvers). It empowers rivals to create engaging challenges, rewards daredevils for their ingenuity, and fosters a culture of learning and collaboration. (and earning money 😉)

## In Progress Features:

- Seamless Deployment: Launched on the dedicated codeduel.com domain for immediate accessibility.
- Automated Rewards & Escrow: Custom Solana programs manage:
    - Question data (status, reward, entrance fee)
    - Rival and daredevil account addresses for automatic reward transfers or fee refunds
    - Paymaster integration for flexible token selection, enhancing user experience
- Multiple Responses: Daredevils can attempt multiple solutions per question, while rivals set response limits.
- Bug Bounty Integration: Solutionless questions attract diverse talent and challenge participants.
- Function as an automated hackathon manager, ensuring code functionality and fair rewards.
- Containerized Code Execution: Execution occurs in isolated containers for enhanced security and accuracy.
- Dynamic Test Cases: Separate test cases from executable and executor code for transparency and maintainability.
- Solana Program Support: Prioritize features that unlock the potential of the Solana ecosystem.
- Language Agnostic Platform: Long-term goal to support all programming languages for question design and solving.
- Intuitive UI/UX: User-friendly interface for both rivals and daredevils, accessible yet feature-rich.

## Technical Workflow:

- Rival Question Creation:
    - Submit question parameters: Title, description, deadline, status, category, reward, entrance fee, sample solution, executor code, test inputs, and test outputs.
    - Send reward amount to platform's secure escrow account.
    - Question becomes visible to daredevils.
    
- Daredevil Submission:
    - Provide question ID and their own executable solution code.
    - Solution is executed by rival's executor code in a container.
    - If correct, question status changes to "CLOSED_SOLVED", reward and entrance fee (minus platform fee) are transferred to the daredevil, and remaining entrance fees are refunded to the rival.

## Current Limitations:

- Only SOL tokens are supported for rewards and entrance fees.
- Test inputs and outputs are not yet fully practical.
- Supporting all programming languages for questions and solutions is a long-term goal.

## Development Roadmap:

Kindly refer to [Future Features](./future_features.md).

## Contributing:

We welcome contributions from the community! Please create your branch and open a pull request explaining your improvements.

## License:

This project is licensed under the MIT License. See the LICENSE file for details.



> Thank you for your interest in CodeDuel! We believe it has the potential to transform the way programmers learn, compete, and collaborate. By joining our community, you can play a vital role in making this vision a reality.
