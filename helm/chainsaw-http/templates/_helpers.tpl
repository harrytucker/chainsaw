{{/*
 Expand the name of the chart.
 */}}
{{- define "chainsaw-http.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
 Create a default fully qualified app name.
 We truncate at 63 chars because some Kubernetes name fields are limited to this (by the DNS naming spec).
 If release name contains chart name it will be used as a full name.
 */}}
{{- define "chainsaw-http.fullname" -}}
{{- if .Values.fullnameOverride }}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- $name := default .Chart.Name .Values.nameOverride }}
{{- if contains $name .Release.Name }}
{{- .Release.Name | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" }}
{{- end }}
{{- end }}
{{- end }}

{{/*
 Create chart name and version as used by the chart label.
 */}}
{{- define "chainsaw-http.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
 Istio labels
 */}}
{{- define "chainsaw-http.istioLabels" -}}
app: {{ include "chainsaw-http.fullname" . }}
version: {{ .Chart.AppVersion | quote }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "chainsaw-http.labels" -}}
helm.sh/chart: {{ include "chainsaw-http.chart" . }}
{{ include "chainsaw-http.selectorLabels" . }}
{{ include "chainsaw-http.istioLabels" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{/*
Selector labels
*/}}
{{- define "chainsaw-http.selectorLabels" -}}
{{ include "chainsaw-http.istioLabels" . }}
app.kubernetes.io/name: {{ include "chainsaw-http.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{/*
Create the name of the service account to use
*/}}
{{- define "chainsaw-http.serviceAccountName" -}}
{{- if .Values.serviceAccount.create }}
{{- default (include "chainsaw-http.fullname" .) .Values.serviceAccount.name }}
{{- else }}
{{- default "default" .Values.serviceAccount.name }}
{{- end }}
{{- end }}
