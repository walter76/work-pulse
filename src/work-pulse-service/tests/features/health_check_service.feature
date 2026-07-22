Feature: Health Check Service

  Scenario: Health check endpoint returns 200 OK
    Given the health check service is running
    When I send a GET request to "/api/v1/health"
    Then the response status code should be 200
    And the service should be healthy
    And the database should be disabled
