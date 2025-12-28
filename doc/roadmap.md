```mermaid
gantt
    title Pixrium Roadmap for 1.0.0
    dateFormat  YYYY-MM-DD
    axisFormat  %m/%d

    section 準備
        プロダクトビジョン策定      :a1, 2025-01-01, 5d
        バックログ作成              :a2, after a1, 5d

    section Sprint 1
        スプリント計画              :s1p, 2025-01-15, 1d
        開発・実装                  :s1d, after s1p, 9d
        スプリントレビュー          :s1r, after s1d, 1d
        レトロスペクティブ          :s1t, after s1r, 1d

    section Sprint 2
        スプリント計画              :s2p, 2025-02-01, 1d
        開発・実装                  :s2d, after s2p, 9d
        スプリントレビュー          :s2r, after s2d, 1d
        レトロスペクティブ          :s2t, after s2r, 1d

    section Sprint 3
        スプリント計画              :s3p, 2025-02-15, 1d
        開発・実装                  :s3d, after s3p, 9d
        スプリントレビュー          :s3r, after s3d, 1d
        レトロスペクティブ          :s3t, after s3r, 1d

    section リリース
        リリース準備                :rel1, 2025-03-01, 3d
        本番リリース                :milestone, rel2, after rel1, 1d
```