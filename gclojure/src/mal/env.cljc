(ns mal.env)
(use 'clojure.test)
(use 'clojure.tools.trace)

(defn env-new [env] {:outer env :data (atom {})})

(defn env-set [env key val]
  (swap! (:data env) (fn [old] (assoc old key val))))

(defn env-find [env key] (cond (nil? env) nil
                               (contains? @(:data env) key) env
                               :else (env-find (:outer env) key)))

(defn env-get [env key] (let [found-env (env-find env key)]
                          (if (nil? found-env) nil
                              (get @(:data found-env) key))))