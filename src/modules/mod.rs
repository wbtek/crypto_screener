
// The MIT License (MIT)
//
// Copyright (c) 2024 Greg Slocum, WBTek,
// a division of WhiteBear Family, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! # WBTek Crypto Screener Application Modules
//!
//! This file defines the main structure of the WBTek Crypto Screener application by organizing 
//! and exposing various modules that handle different aspects of the app's functionality.
//!
//! ## Module Overview
//!
//! - `component`: Contains the primary `Model` component, managing the application’s main state and UI.
//! - `about`: Defines the "About" modal view, providing information about the application and its usage.
//! - `button`: Implements the `HeaderButton` component, used for sortable table headers.
//! - `cryptodata`: Defines the `CryptoData` struct, which models cryptocurrency data fetched from an external API.
//! - `headview`: Provides functionality for rendering the table header, including sorting interactions.
//! - `fetch`: Contains functions for fetching data from the cryptocurrency API.
//! - `implmodel`: Adds additional methods and utilities to the main `Model` component.
//! - `message`: Defines messages (`Msg`) used to manage application state updates and user interactions.
//! - `rowview`: Handles the rendering of rows within the cryptocurrency data table.
//! - `sort`: Provides utilities for sorting data based on various criteria.
//! - `utils`: Contains miscellaneous utility functions used throughout the application.
//!
//! Each module focuses on a specific part of the application's functionality, promoting organized
//! and maintainable code structure. Together, they build a comprehensive interface for interacting
//! with cryptocurrency data in a Yew application.

pub mod component;

mod about;
mod button;
mod cryptodata;
mod headview;
mod fetch;
mod implmodel;
mod message;
mod rowview;
mod sort;
mod utils;

