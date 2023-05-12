// SPDX-FileCopyrightText: Copyright (c) 2017-2023 slowtec GmbH <post@slowtec.de>
// SPDX-License-Identifier: MIT OR Apache-2.0

use async_trait::async_trait;

#[async_trait]
/// A Modbus server service.
pub trait Service {
    /// Requests handled by the service.
    type Request;

    /// Responses given by the service.
    type Response;

    /// Errors produced by the service.
    type Error;

    /// The result type.
    //type Result: Result<Self::Response, Self::Error>;

    /// Process the request and return the response asynchronously.
    async fn call(&self, req: Self::Request) -> Result<Self::Response, Self::Error>;
}
