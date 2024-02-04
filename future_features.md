# Main Plans

### Deployment
- Our primary objective is to deploy our platform on the `codeduel.com` domain and officially launch the project.

### Automating the Rewarding Process
- One of our most crucial goals is to develop custom Solana programs for CodeDuel that will facilitate the following functionalities:
  - Keeping track of each question, including its current status, associated reward, and required entrance fee.
  - Monitoring the rival (question designer) and daredevil (question solver) account addresses to enable automatic transfer of rewards to the winner or refunding the question designer if the question deadline is exceeded.
  - Implementing a "pay master" feature within the program to support arbitrary token selection for rewards and entrance fees, ensuring a seamless user experience for platform contributors.

# Technical Plans

### Multiple Responses
- We aim to provide the functionality for daredevils to attempt multiple responses until the correct one is accepted. Additionally, rivals will have the ability to set a threshold for the limit of responses per daredevil. Currently, each daredevil can only attempt one question, but we plan to introduce the multiple responses feature for those who believe in second chances.

### Bug Bounty Questions
- At present, the platform's contributors are primarily developers who know the answers to their questions. We intend to support the inclusion of solutionless questions on the platform.
- This will allow corporations to utilize our platform as an automated hackathon manager, ensuring that the code provided by hackers is functional and that hackers are rewarded regardless of their experience, geographical location, or other potential limitations.

### Containerizing the Code Executioners
- Due to time constraints during the hackathon, the code execution component of the platform is currently basic, and the executable program provided by the user has access to the server.
- Prior to deployment, we will implement a feature that allows executables to run in an isolated container, ensuring both host and guest security and accuracy.

### Dynamic Test Cases
- The current separate test cases for questions are not practical, and users must hardcode the desired testing values for the executable code within the executor code.
- By generating a trait object for bug bounty questions or obtaining a prepared trait object from the user, we can separate the test cases from the executable and executor codes, ensuring accuracy and transparency.

### Supporting Solana Programs
- The platform's primary supporting technologies are Rust-based programs and Solana programs, which are commonly written in Rust.
- Due to time constraints, we were unable to handle the execution process of Solana programs, but achieving this remains one of our primary goals.

### Supporting All Stacks
- Our long-term objective is to support all programming languages for designing and solving questions.
- This means that a rival can design a question in language A, and another daredevil can solve it using programming language B by sharing the associated interface of each program.

# Additional Improvements

### Improving the UI
- We will significantly enhance the platform's UI and UX to ensure ease of use while maintaining the platform's complex logic and technologies.