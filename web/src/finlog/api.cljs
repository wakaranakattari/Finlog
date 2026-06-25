;; @file    <finlog/api.cljs>
;; @author  <wakaranakattari@gmail.com>
;; @info    <api functions for loading expenses and username from server>
;; @version <1.0>

;; @secstart->@secname <nsrq>
(ns finlog.api
  (:require [cljs.core.async.interop :refer-macros [<p!]]
            [cljs.core.async :refer [go]]))
;; @secend->@secname <nsrq>

;; @secstart->@secname <loadexpenses>
  ;; @funcinfo <fetches expenses json from server, returns vector of maps or empty vector on error>
(defn load-expenses []
  (go
    (try
      (let [response (<p! (js/fetch "/data/expenses/expensive.json"))]
        (if (.-ok response)
          ;; @info <parse and convert json to clj map with keyword keys>
          (js->clj (<p! (.json response)) :keywordize-keys true)
          ;; @info <return empty vector if response is not ok>
          []))
      ;; @info <return empty vector on any network or parse error>
      (catch :default _ []))))
;; @secend->@secname <loadexpenses>

;; @secstart->@secname <loadusername>
  ;; @funcinfo <fetches username from config json, returns name string or "User" on error>
(defn load-username []
  (go
    (try
      (let [response (<p! (js/fetch "/data/config/config.json"))
            data     (js->clj (<p! (.json response)) :keywordize-keys true)]
        ;; @info <fall back to "User" if name key is missing>
        (or (:name data) "User"))
      ;; @info <return default username on any error>
      (catch :default _ "User"))))
;; @secend->@secname <loadusername>