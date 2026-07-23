Feature: Health Check Service

  Scenario: Health check endpoint returns 200 OK when database is disabled
    Given the health check service is running
    When I send a GET request to "/api/v1/health"
    Then the response status code should be 200
    And the service should be healthy
    And the database should be disabled

  Scenario: Health check endpoint returns 200 OK when database is connected
    Given the health check service is running with a connected database
    When I send a GET request to "/api/v1/health"
    Then the response status code should be 200
    And the service should be healthy
    And the database should be connected

  Scenario: Health check endpoint returns 503 when database is disconnected
    Given the health check service is running with a disconnected database
    When I send a GET request to "/api/v1/health"
    Then the response status code should be 503
    And the service should be unhealthy
    And the database should be disconnected

  Scenario: Non-existing health sub-route returns 404
    Given the health check service is running
    When I send a GET request to "/api/v1/health/unknown"
    Then the response status code should be 404
